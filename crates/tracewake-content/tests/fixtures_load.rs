use std::collections::BTreeSet;

use tracewake_content::fixtures;
use tracewake_content::load::load_fixture_package;
use tracewake_content::schema::InitialBeliefSchema;
use tracewake_content::serialization::{deserialize_fixture, serialize_fixture};
use tracewake_content::validate::validate_fixture;
use tracewake_core::actions::ActionRegistry;
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{Confidence, Proposition, SourceRef};
use tracewake_core::ids::{BeliefId, ContainerId, ContentManifestId, ContentVersion, EventId};
use tracewake_core::location::Location;
use tracewake_core::time::SimTick;

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry.register_phase2a_epistemics();
    registry
}

#[test]
fn all_fixtures_load_deterministically_and_validate() {
    let registry = registry();
    let all = fixtures::all();
    assert_eq!(all.len(), 13);

    let ids = all
        .iter()
        .map(|fixture| fixture.fixture.fixture_id.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        ids,
        BTreeSet::from([
            "container_item_move_001".to_string(),
            "debug_attach_001".to_string(),
            "door_access_001".to_string(),
            "expectation_contradiction_001".to_string(),
            "knowledge_blocker_accuse_001".to_string(),
            "no_human_advance_001".to_string(),
            "no_human_epistemic_check_001".to_string(),
            "possession_parity_001".to_string(),
            "replay_item_location_001".to_string(),
            "sound_uncertainty_001".to_string(),
            "strongbox_001".to_string(),
            "view_filtering_001".to_string(),
            "view_model_local_actions_001".to_string(),
        ])
    );

    for golden in all {
        validate_fixture(&golden.fixture, &registry).unwrap();
        let first = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )
        .unwrap();
        let second = load_fixture_package(
            ContentManifestId::new(format!("manifest_{}", golden.fixture.fixture_id.as_str()))
                .unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![golden.source_file()],
        )
        .unwrap();
        assert_eq!(first.canonical_world, second.canonical_world);
        assert_eq!(
            first.manifest.content_fingerprint,
            second.manifest.content_fingerprint
        );
    }
}

#[test]
fn strongbox_fixture_preserves_spec_0001_roster_and_physical_coin() {
    let golden = fixtures::strongbox_001();
    let actor_ids = golden
        .fixture
        .actors
        .iter()
        .map(|actor| actor.actor_id.as_str())
        .collect::<BTreeSet<_>>();
    let place_ids = golden
        .fixture
        .places
        .iter()
        .map(|place| place.place_id.as_str())
        .collect::<BTreeSet<_>>();

    assert!(actor_ids.contains("actor_tomas"));
    assert!(actor_ids.contains("actor_elena"));
    assert!(place_ids.contains("house_tomas"));
    assert!(golden
        .fixture
        .containers
        .iter()
        .any(|container| container.container_id.as_str() == "strongbox_tomas"));
    assert!(golden.fixture.items.iter().any(|item| {
        item.item_id.as_str() == "coin_stack_01"
            && item.location == Location::InContainer(ContainerId::new("strongbox_tomas").unwrap())
    }));
    let seed = golden
        .fixture
        .initial_beliefs
        .iter()
        .find(|belief| {
            belief.belief_id.as_str() == "belief_tomas_expects_coin_stack_01_in_strongbox_tomas"
        })
        .expect("Tomas expectation seed exists");
    assert_eq!(seed.source_kind.stable_id(), "authored_prehistory");
    match &seed.source {
        SourceRef::Event(event_id) => assert_eq!(
            event_id.as_str(),
            "prehistory_tomas_checked_strongbox_before_start"
        ),
        other => panic!("authored prehistory seed should be event-backed, got {other:?}"),
    }
}

#[test]
fn phase2a_fixtures_are_registered_and_script_free() {
    let all = fixtures::all();
    let phase2a_ids = [
        "strongbox_001",
        "expectation_contradiction_001",
        "possession_parity_001",
        "view_filtering_001",
        "knowledge_blocker_accuse_001",
        "sound_uncertainty_001",
        "no_human_epistemic_check_001",
    ];
    for fixture_id in phase2a_ids {
        let golden = all
            .iter()
            .find(|golden| golden.fixture.fixture_id.as_str() == fixture_id)
            .expect("Phase 2A fixture registered");
        validate_fixture(&golden.fixture, &registry()).unwrap();
        let serialized = String::from_utf8(golden.source_file().bytes).unwrap();
        for forbidden in ["culprit", "stolen_flag", "npc_knows_truth", "quest_state"] {
            assert!(
                !serialized.contains(forbidden),
                "{fixture_id} contains forbidden shortcut {forbidden}"
            );
        }
    }
}

#[test]
fn possession_parity_contains_mara_as_ordinary_actor() {
    let golden = fixtures::possession_parity_001();

    assert!(golden
        .fixture
        .actors
        .iter()
        .any(|actor| actor.actor_id.as_str() == "actor_mara"));
    assert!(golden
        .contract
        .acceptance_assertions
        .iter()
        .any(|assertion| assertion.contains("no culprit")));
}

#[test]
fn every_fixture_declares_contract_actions_reports_and_assertions() {
    for golden in fixtures::all() {
        assert_eq!(
            golden.contract.fixture_id,
            golden.fixture.fixture_id.as_str()
        );
        assert!(!golden.contract.setup.is_empty());
        assert!(!golden.contract.allowed_actions.is_empty());
        assert!(!golden.contract.expected_events_or_reports.is_empty());
        assert!(!golden.contract.acceptance_assertions.is_empty());
    }
}

#[test]
fn valid_epistemic_seed_validates_and_round_trips_canonically() {
    let mut golden = fixtures::strongbox_001();
    golden
        .fixture
        .initial_beliefs
        .push(InitialBeliefSchema::new_expectation(
            BeliefId::new("belief_tomas_expected_coin").unwrap(),
            "actor_tomas".parse().unwrap(),
            Proposition::ItemLocatedInContainer {
                item_id: "coin_stack_01".parse().unwrap(),
                container_id: "strongbox_tomas".parse().unwrap(),
            },
            Confidence::new(900).unwrap(),
            SourceRef::Event(EventId::new("event_authored_prehistory_tomas_coin").unwrap()),
            SimTick::ZERO,
        ));
    golden.fixture.canonicalize();

    validate_fixture(&golden.fixture, &registry()).unwrap();
    let first = serialize_fixture(&golden.fixture);
    let parsed = deserialize_fixture(&first).unwrap();
    let second = serialize_fixture(&parsed);

    assert_eq!(first, second);
    let seed = parsed
        .initial_beliefs
        .iter()
        .find(|belief| belief.belief_id.as_str() == "belief_tomas_expected_coin")
        .expect("added seed round-trips");
    assert_eq!(seed.schema_version.as_str(), EPISTEMIC_RECORD_SCHEMA_V1);
    assert!(String::from_utf8(first)
        .unwrap()
        .contains("initial_belief|belief_tomas_expected_coin|actor_tomas"));
}
