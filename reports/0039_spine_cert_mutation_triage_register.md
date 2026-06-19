# 0039 SPINE-CERT Mutation Triage Register

Date: 2026-06-18
Implementation SHA: `f785c7d43d288f830201873c32e376ae8340adf8`
Spec: `archive/specs/0039_SPINE_CERT_MUTATION_REMEDIATION_AND_REPLACEMENT_CERTIFICATION_SPEC.md`
Ticket: `archive/tickets/0039SPICERMUT-020.md`

## Summary

The standing SPINE-CERT mutation perimeter was enumerated and run under the
checked-in cargo-mutants configuration after correcting the cargo-mutants
27.1.0 workspace-test posture:

- `.cargo/mutants.toml` now uses `test_workspace = true` plus
  `additional_cargo_args = ["--locked"]`. The earlier
  `additional_cargo_args = ["--workspace", "--locked"]` duplicated Cargo's
  workspace flag and made every mutant unviable.
- `cargo mutants --workspace --list-files` enumerated 48 files.
- `cargo mutants --workspace --list` enumerated 2625 mutants.
- `cargo mutants --workspace --no-shuffle -j 8` completed the full denominator:
  2625 tested, 37 missed, 2040 caught, 545 unviable, 3 timeouts.
- Timeout retry with `cargo mutants --workspace --no-shuffle -j 1 --timeout 600`
  over the timed-out identities completed with 0 timeouts: 2 missed, 16 caught,
  2 unviable.

Ticket 020 certification disposition: `SPINE-CERT scoped remediation`. At that
point there were 38 actionable missed mutants and 0 unresolved timeouts after
retry. No missed mutant was accepted as equivalent or non-critical, and
`.cargo/mutants-baseline-misses.txt` remained unchanged/empty.

Capstone disposition after tickets 022, 023, and 024: `SPINE-CERT passed` for
the scoped 0039 replacement artifact. The final standing run at implementation
commit `92ba47f14998e0ea2fc95502bc3b76c5909478ca` tested the same 2625-mutant
denominator with 2079 caught, 545 unviable, 0 missed, and 1 timeout; the timeout
retry completed with 6 caught, 2 unviable, 0 missed, and 0 timeouts. No blocked
or untriaged survivor remains.

## Evidence Artifacts

| Artifact | Purpose |
| --- | --- |
| `reports/0039_spine_cert_mutation_list_files.txt` | Standing perimeter file census, 48 files |
| `reports/0039_spine_cert_mutation_list.txt` | Standing mutant census, 2625 mutants |
| `reports/0039_spine_cert_mutation_run.txt` | Full standing run transcript summary |
| `reports/0039_spine_cert_mutation_missed.txt` | 37 missed mutants from the full run |
| `reports/0039_spine_cert_mutation_timeout.txt` | 3 full-run timeout identities |
| `reports/0039_spine_cert_mutation_timeout_retry.txt` | Timeout retry transcript |
| `reports/0039_spine_cert_mutation_timeout_retry_missed.txt` | 2 missed identities from timeout retry |
| `reports/0039_spine_cert_mutation_timeout_retry_caught.txt` | 16 retry-caught identities |

## Fingerprints

```text
0726fc1470a5b2d9d5625394bf091105103f662c0ac5219b09cfb618af44c5d6  .cargo/mutants.toml
5f20abf363f08b7369b6975ef25ee9398ee7c397fb32214cbfb03f1f310bbcc2  .github/workflows/ci.yml
f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59  Cargo.lock
b3d3a0b60f6607976db84cad78e9695160f52db72fae1c4c870445ef6afa5e9f  reports/0039_spine_cert_mutation_list_files.txt
b6551f30294878a5f7b452767eb2ace63f9a9add7f62aa21e3e9ab66d349ff36  reports/0039_spine_cert_mutation_list.txt
a1b90724b4eee09962e68994151c2e6b15c24a4a1e2fa4c61bd5009d66316227  reports/0039_spine_cert_mutation_run.txt
3eaaed92fc65d54188fe08caaf96c72f17248a82b8fc23dbec5c8733d59c7248  reports/0039_spine_cert_mutation_timeout_retry.txt
96368c7e04937a54defc4ec5e2fc737e20b3253a7abf6657ac9b5abeb702f2cf  reports/0039_spine_cert_mutation_missed.txt
5ad44c5cfdbf85cecf92e429e1e9eadff4c741875e035d8fdfff0f216217655e  reports/0039_spine_cert_mutation_timeout.txt
0fca98d33b430c93bbb007e930c92d98d6117a12c01398ab82dabea6ce03edda  reports/0039_spine_cert_mutation_timeout_retry_missed.txt
e5a7d20275fe8e0fddc2352d26dc7e18186fef8b859bc767c38b4260f19db041  reports/0039_spine_cert_mutation_timeout_retry_caught.txt
```

## Reconciliation

Historical Wave B survivors from `reports/0038_spine_cert_mutation_triage_register.md`
were reconciled through tickets `0039SPICERMUT-002` through
`0039SPICERMUT-019`, each of which ran a per-file mutation proof under
`--no-config` because ticket 001 installed the expanded standing config. The
standing run in this ticket is the final denominator and supersedes the
historical 296 count for remaining remediation.

## Survivor Register

Every row below has:

- Tool outcome: `missed`.
- Certified reachability: behavior-relevant unless a future reviewer proves
  equivalent/non-critical with evidence.
- Certification disposition: routed remediation.
- Review signoff: not accepted for certification.

| Responsible ticket | Responsible seam | Count | Evidence |
| --- | --- | ---: | --- |
| `0039SPICERMUT-022` | controller possession/debug authorization | 5 | `reports/0039_spine_cert_mutation_missed.txt` |
| `0039SPICERMUT-023` | authoritative state accessors and door connectivity | 4 | `reports/0039_spine_cert_mutation_missed.txt` |
| `0039SPICERMUT-024` | epistemic projection checksum, actor-known projection, location/source serialization | 29 full-run misses + 1 retry miss | `reports/0039_spine_cert_mutation_missed.txt`, `reports/0039_spine_cert_mutation_timeout_retry_missed.txt` |

### Controller Survivors

```text
crates/tracewake-core/src/controller.rs:115:9: replace ControllerBindings::authorize -> Result<(), ControllerError> with Ok(())
crates/tracewake-core/src/controller.rs:83:28: replace += with *= in ControllerBindings::detach
crates/tracewake-core/src/controller.rs:50:28: replace += with *= in ControllerBindings::attach
crates/tracewake-core/src/controller.rs:107:9: replace ControllerBindings::debug_bindings -> Vec<&RuntimeControllerBinding> with vec![]
crates/tracewake-core/src/controller.rs:83:28: replace += with -= in ControllerBindings::detach
```

### State Survivors

```text
crates/tracewake-core/src/state.rs:335:9: replace AgentState::need_threshold_crossings -> &BTreeMap<crate::ids::EventId, NeedThresholdCrossingRecord> with Box::leak(Box::new(BTreeMap::new()))
crates/tracewake-core/src/state.rs:347:9: replace AgentState::candidate_goal_evaluations -> &BTreeMap<crate::ids::EventId, CandidateGoalEvaluationRecord> with Box::leak(Box::new(BTreeMap::new()))
crates/tracewake-core/src/state.rs:422:9: replace DoorState::connects_place -> bool with true
crates/tracewake-core/src/state.rs:422:58: replace == with != in DoorState::connects_place
```

### Epistemic Projection Survivors

```text
crates/tracewake-core/src/epistemics/projection.rs:292:9: replace EpistemicProjectionChecksum::as_str -> &str with "xyzzy"
crates/tracewake-core/src/epistemics/projection.rs:299:22: replace ^= with |= in EpistemicProjectionChecksum::from_canonical_lines
crates/tracewake-core/src/epistemics/projection.rs:343:38: replace += with *= in EpistemicProjection::record_applied_event
crates/tracewake-core/src/epistemics/projection.rs:343:9: replace EpistemicProjection::record_applied_event with ()
crates/tracewake-core/src/epistemics/projection.rs:442:9: replace EpistemicProjection::has_belief -> bool with true
crates/tracewake-core/src/epistemics/projection.rs:446:9: replace EpistemicProjection::is_empty -> bool with true
crates/tracewake-core/src/epistemics/projection.rs:450:13: replace && with || in EpistemicProjection::is_empty
crates/tracewake-core/src/epistemics/projection.rs:447:13: replace && with || in EpistemicProjection::is_empty
crates/tracewake-core/src/epistemics/projection.rs:449:13: replace && with || in EpistemicProjection::is_empty
crates/tracewake-core/src/epistemics/projection.rs:448:13: replace && with || in EpistemicProjection::is_empty
crates/tracewake-core/src/epistemics/projection.rs:499:9: replace EpistemicProjection::notebook_entries_for_context -> Vec<&NotebookEntry> with vec![]
crates/tracewake-core/src/epistemics/projection.rs:744:9: replace ActorKnownProjectionSource::stable_id -> &'static str with "xyzzy"
crates/tracewake-core/src/epistemics/projection.rs:744:9: replace ActorKnownProjectionSource::stable_id -> &'static str with ""
crates/tracewake-core/src/epistemics/projection.rs:896:9: replace ActorKnownProjectionRecord::serialize_canonical -> String with String::new()
crates/tracewake-core/src/epistemics/projection.rs:896:9: replace ActorKnownProjectionRecord::serialize_canonical -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/projection.rs:1193:5: replace workplace_record_is_newer -> bool with true
crates/tracewake-core/src/epistemics/projection.rs:1193:29: replace > with >= in workplace_record_is_newer
crates/tracewake-core/src/epistemics/projection.rs:1195:13: replace && with || in workplace_record_is_newer
crates/tracewake-core/src/epistemics/projection.rs:1195:44: replace > with >= in workplace_record_is_newer
crates/tracewake-core/src/epistemics/projection.rs:1203:8: delete ! in reclassifying_record_freshness
crates/tracewake-core/src/epistemics/projection.rs:1222:5: replace location_key -> String with String::new()
crates/tracewake-core/src/epistemics/projection.rs:1222:5: replace location_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/projection.rs:1410:9: delete match arm Some("visible_actor") in actor_known_records_from_observation
crates/tracewake-core/src/epistemics/projection.rs:1500:9: delete match arm "place" in observation_item_location
crates/tracewake-core/src/epistemics/projection.rs:1514:5: replace holder_key -> String with String::new()
crates/tracewake-core/src/epistemics/projection.rs:1514:5: replace holder_key -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/projection.rs:1561:5: replace source_summary -> String with String::new()
crates/tracewake-core/src/epistemics/projection.rs:1561:5: replace source_summary -> String with "xyzzy".into()
crates/tracewake-core/src/epistemics/projection.rs:292:9: replace EpistemicProjectionChecksum::as_str -> &str with ""
```

## Timeout Retry Disposition

The full run timed out on three identities:

```text
crates/tracewake-core/src/agent/perception.rs:201:42: replace && with || in current_place_perception_events
crates/tracewake-core/src/agent/perception.rs:548:38: delete ! in visible_item_payload
crates/tracewake-core/src/epistemics/projection.rs:292:9: replace EpistemicProjectionChecksum::as_str -> &str with ""
```

Retry command:

```sh
cargo mutants --workspace --no-shuffle -j 1 --timeout 600 -F 'current_place_perception_events|visible_item_payload|EpistemicProjectionChecksum::as_str' -o mutants-timeout-retry.out
```

Retry outcome: 20 mutants tested in 3m: 2 missed, 16 caught, 2 unviable, 0
timeouts. The two perception timeout identities were caught on retry. The
checksum empty-string identity is included in `0039SPICERMUT-024`.

## Capstone Reconciliation Addendum

After archived tickets `0039SPICERMUT-022`, `0039SPICERMUT-023`, and
`0039SPICERMUT-024` landed, the capstone reran the standing perimeter:

```sh
cargo mutants --workspace --list-files
cargo mutants --workspace --list
cargo mutants --workspace --no-shuffle -j 8 -o mutants-final-0039.out
cargo mutants --workspace --no-shuffle -j 1 --timeout 600 -F 'generate_candidate_goals' -o mutants-final-0039-timeout-retry.out
```

Final standing census:

- `cargo mutants --workspace --list-files`: 48 files.
- `cargo mutants --workspace --list`: 2625 mutants.
- Full run: 2625 mutants tested in 3h; 2079 caught, 545 unviable, 0 missed, 1 timeout.
- Retry: 8 filtered mutants tested in 68s; 6 caught, 2 unviable, 0 missed, 0 timeouts.

The only full-run timeout was:

```text
crates/tracewake-core/src/agent/generation.rs:101:13: delete match arm (NeedKind::Fatigue, NeedBand::Urgent) in generate_candidate_goals
```

The retry resolved that timeout. `missed.txt` was empty in the final standing
run and in the timeout retry. The 38 actionable missed mutants from ticket 020
are therefore reconciled as killed by remediation and final standing proof:

| Responsible ticket | Count | Final disposition |
| --- | ---: | --- |
| `0039SPICERMUT-022` | 5 | killed; final standing run has zero missed controller mutants |
| `0039SPICERMUT-023` | 4 | killed; final standing run has zero missed state mutants |
| `0039SPICERMUT-024` | 29 full-run misses + 1 retry miss | killed; final standing run has zero missed projection mutants |

No survivor was accepted as equivalent or non-critical. No baseline-miss file was
used to launder a behavior-changing survivor. The capstone replacement
acceptance artifact is
`archive/reports/0039_spine_cert_mutation_remediation_replacement_certification_acceptance.md`.
