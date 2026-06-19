# 0040 EPI-CERT Mutation Triage Register

Date: 2026-06-19
Ticket: `archive/tickets/0040EPICERHOL-014.md`
Spec: `archive/specs/0040_EPI_CERT_HOLDER_KNOWN_CONTEXTS_BELIEFS_OBSERVATIONS_PROVENANCE_POSSESSION_PARITY_VIEW_MODELS_AND_DEBUG_QUARANTINE_CERTIFICATION_SPEC.md`

## Summary

The checked-in cargo-mutants configuration was expanded additively with
`crates/tracewake-core/src/epistemics/**`, increasing the configured file census
from 48 files to 54 files and the Wave B mutant census to 2763 mutants.

Final Wave B: `cargo mutants --workspace --no-shuffle -j 8 -o reports/0040_epi_cert_mutation_wave_b_j8.out`

- 2763 tested
- 2143 caught
- 589 unviable
- 27 missed
- 4 timeouts

Timeout retry: `cargo mutants --workspace --no-shuffle -j 1 --timeout 600 -F 'current_place_perception_events|Confidence::parts_per_thousand|PropositionParseError|PropositionReferenceError' -o reports/0040_epi_cert_mutation_wave_b_timeout_retry.out`

- 22 tested
- 10 caught
- 5 unviable
- 7 missed
- 0 timeouts

Final unique missed-mutant floor: 30. No missed mutant is accepted as equivalent or non-critical. Certification disposition: `EPI-CERT scoped remediation`.

## Evidence Artifacts

| Artifact | Purpose |
|---|---|
| `reports/0040_epi_cert_mutation_wave_a_list_files.txt` | Wave A standing file census, 48 files |
| `reports/0040_epi_cert_mutation_wave_b_list_files.txt` | Expanded Wave B file census, 54 files |
| `reports/0040_epi_cert_mutation_wave_b_list.txt` | Expanded Wave B mutant census, 2763 mutants |
| `reports/0040_epi_cert_mutation_wave_b_missed.txt` | Full-run missed mutants, 27 |
| `reports/0040_epi_cert_mutation_wave_b_timeout.txt` | Full-run timeout identities, 4 |
| `reports/0040_epi_cert_mutation_timeout_retry_missed.txt` | Timeout retry missed mutants, 7 |
| `reports/0040_epi_cert_mutation_timeout_retry_timeout.txt` | Timeout retry timeouts, 0 |
| `reports/0040_epi_cert_mutation_final_missed.txt` | Unique final missed-mutant floor, 30 |

## Fingerprints

```text
20868a3a2ed304fe5eb87bb86a4f27ebc1c3915d0a3f347ebe79d03ad064b01b  .cargo/mutants.toml
5f20abf363f08b7369b6975ef25ee9398ee7c397fb32214cbfb03f1f310bbcc2  .github/workflows/ci.yml
f207f6208d64c1a7cce0daa27c00add40a52edb99b1a6ce3a1301cb44ef60a59  Cargo.lock
b3d3a0b60f6607976db84cad78e9695160f52db72fae1c4c870445ef6afa5e9f  reports/0040_epi_cert_mutation_wave_a_list_files.txt
9f3335f0733d969537595a77e1af0d61e1faf3c92561f1e79f97afbbac917e9d  reports/0040_epi_cert_mutation_wave_b_list_files.txt
0c3391bb8510028fcbfe53f035cbf938ed768adc9494112fa0a629ba60e99b5a  reports/0040_epi_cert_mutation_wave_b_list.txt
e2788d0f4974d839d264537e4689eebe9820f96e9f328f93184e5a3b9bfdc9cf  reports/0040_epi_cert_mutation_wave_b_missed.txt
d4bd53fc81520daecc2a1fefb165c8a2fabd6160b26f0b45959f0fb936fffa0c  reports/0040_epi_cert_mutation_wave_b_timeout.txt
bf83a6e1001807beddf1f51d95208d16aea1b9d8217523c47513049c36b6e9ea  reports/0040_epi_cert_mutation_final_missed.txt
7b8b00759482de66a3a40ec421028ee0f298e4d72d265a1962a6b7201f1a799d  reports/0040_epi_cert_mutation_timeout_retry_missed.txt
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  reports/0040_epi_cert_mutation_timeout_retry_timeout.txt
```

## Register

Every row has tool outcome `missed`, certified reachability `behavior-relevant until remediated`, behavior witness `missing`, negative/contamination control `not yet killing this mutant`, disposition `routed remediation`, and review signoff `not accepted for certification`.

| Mutant identity | EPI cross-ref | Responsible layer | Evidence reference |
|---|---|---|---|
| `belief.rs:150:9 stale_after_tick -> None` | `EPI-02`, `EPI-06` | projection/replay; freshness policy | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `belief.rs:150:9 stale_after_tick -> Some(Default::default())` | `EPI-02`, `EPI-06` | projection/replay; freshness policy | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `belief.rs:154:9 observation_ids -> empty set` | `EPI-02`, `EPI-03`, `EPI-05` | projection/replay; provenance linkage | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `belief.rs:158:9 contradiction_ids -> empty set` | `EPI-02`, `EPI-04` | projection/replay; contradiction linkage | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `contradiction.rs:28:9 is_active -> true` | `EPI-04` | projection/replay; contradiction state | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `contradiction.rs:127:13 \|\| -> && in detect_expected_absences` | `EPI-04` | projection/replay; absence contradiction detection | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `observation.rs:25:9 parts_per_thousand -> 0` | `EPI-03` | projection/replay; observation confidence | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `observation.rs:25:9 parts_per_thousand -> 1` | `EPI-03` | projection/replay; observation confidence | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `observation.rs:29:9 is_low -> true` | `EPI-03` | projection/replay; observation confidence | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:260:13 delete at-place contradiction arm` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:266:26 == -> !=` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:266:45 && -> \|\|` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:266:57 == -> !=` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:267:13 delete carried-by contradiction arm` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:273:26 == -> !=` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:273:45 && -> \|\|` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:273:57 == -> !=` | `EPI-04` | projection/replay; proposition contradiction | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:281:9 Display for Proposition -> Ok(default)` | `EPI-02`, `EPI-04`, `EPI-06` | projection/replay; canonical proposition rendering | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:301:13 delete item_carried_by_actor parse arm` | `EPI-02`, `EPI-06` | projection/replay; proposition parsing | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:305:13 delete container_contents_observed parse arm` | `EPI-03`, `EPI-04` | projection/replay; proposition parsing | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:319:13 delete possible_movement_near_place parse arm` | `EPI-07`, `EPI-09` | proposal construction; view-model rendering | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:349:9 delete at_place deserialize arm` | `EPI-02`, `EPI-04` | projection/replay; location parsing | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:351:9 delete carried_by deserialize arm` | `EPI-02`, `EPI-04` | projection/replay; location parsing | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:357:5 render_location -> xyzzy` | `EPI-02`, `EPI-06`, `EPI-09` | projection/replay; view-model rendering | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:357:5 render_location -> empty` | `EPI-02`, `EPI-06`, `EPI-09` | projection/replay; view-model rendering | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:370:5 validate_location -> Ok(())` | `EPI-02`, `EPI-05` | content/schema validation; projection/replay | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:392:5 require_place -> Ok(())` | `EPI-02`, `EPI-05` | content/schema validation; projection/replay | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:403:5 require_container -> Ok(())` | `EPI-03`, `EPI-05` | content/schema validation; projection/replay | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:51:9 Display for PropositionReferenceError -> Ok(default)` | `EPI-02`, `EPI-05` | content/schema validation; diagnostics | `reports/0040_epi_cert_mutation_final_missed.txt` |
| `proposition.rs:83:9 Display for PropositionParseError -> Ok(default)` | `EPI-02`, `EPI-05` | content/schema validation; diagnostics | `reports/0040_epi_cert_mutation_final_missed.txt` |
