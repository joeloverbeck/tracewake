# 0038 SPINE-01 seed event-log fingerprint capture

This supplemental SPINE-01 evidence table was generated on 2026-06-18 from the
public `tracewake-content` fixture loader and `tracewake-core` checksum APIs at
implementation commit `b4b59c92d126692c9f2fa4c986695b9f2e20db2c`.

Method:

- Load each named fixture through `load_fixture_package(...)`.
- Read `LoadedFixture::seed_event_log`.
- Fingerprint `EventLog::serialize_canonical()` with FNV-1a 64-bit over raw
  canonical bytes.
- Compute physical and agent checksums from the loaded canonical states with
  `compute_physical_checksum(...)` and `compute_agent_state_checksum(...)`.

Scope:

- This table fingerprints the authored seed event logs only. Empty seed logs
  are recorded as empty canonical byte fingerprints and are not overclaimed as
  runtime action traces.
- Runtime appended-event and replay evidence for these fixtures is supplied by
  the SPINE-01 command transcripts cited in the acceptance report, especially
  `golden_fixtures_run`, `event_schema_replay_gates`, and the no-human replay
  tests.

| Fixture | Seed event count | World events | Seed event-log fingerprint | Physical checksum | Agent checksum | First event | Last event |
|---|---:|---:|---|---|---|---|---|
| `replay_item_location_001` | 0 | 0 | `fnv1a64-cbf29ce484222325` | `PhysicalChecksum("twc1-b6eb95112833cd6a")` | `AgentStateChecksum("twa1-e86a21dea98bac4e")` | `none` | `none` |
| `container_item_move_001` | 0 | 0 | `fnv1a64-cbf29ce484222325` | `PhysicalChecksum("twc1-f99eed2e457cc258")` | `AgentStateChecksum("twa1-3e8f1dbc7f409b3e")` | `none` | `none` |
| `door_access_001` | 0 | 0 | `fnv1a64-cbf29ce484222325` | `PhysicalChecksum("twc1-b83af0cd3b1b15e2")` | `AgentStateChecksum("twa1-09da0e6471589aaa")` | `none` | `none` |
| `strongbox_001` | 1 | 0 | `fnv1a64-d19149f4a29559fb` | `PhysicalChecksum("twc1-8cee095e38dc1cd8")` | `AgentStateChecksum("twa1-d8facbe015de90de")` | `initial_belief_seeded` | `initial_belief_seeded` |
| `ordinary_workday_001` | 3 | 0 | `fnv1a64-5d39e32446f941ad` | `PhysicalChecksum("twc1-d3091393a2ef43e1")` | `AgentStateChecksum("twa1-470969d1ccc79970")` | `starting_belief_recorded` | `role_assignment_notice_recorded` |
| `sleep_eat_work_001` | 4 | 0 | `fnv1a64-4a79025bf1de4829` | `PhysicalChecksum("twc1-2dbf14a99735ce59")` | `AgentStateChecksum("twa1-5206952f2ec9c780")` | `starting_belief_recorded` | `role_assignment_notice_recorded` |
| `no_human_day_001` | 15 | 0 | `fnv1a64-a1369e12301b7325` | `PhysicalChecksum("twc1-cecfabc0597e02a3")` | `AgentStateChecksum("twa1-e3af7fee80453d86")` | `starting_belief_recorded` | `role_assignment_notice_recorded` |
