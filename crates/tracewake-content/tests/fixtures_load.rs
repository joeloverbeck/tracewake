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
    registry
}

#[test]
fn all_seven_fixtures_load_deterministically_and_validate() {
    let registry = registry();
    let all = fixtures::all();
    assert_eq!(all.len(), 7);

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
            "no_human_advance_001".to_string(),
            "replay_item_location_001".to_string(),
            "strongbox_001".to_string(),
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

    validate_fixture(&golden.fixture, &registry()).unwrap();
    let first = serialize_fixture(&golden.fixture);
    let parsed = deserialize_fixture(&first).unwrap();
    let second = serialize_fixture(&parsed);

    assert_eq!(first, second);
    assert_eq!(parsed.initial_beliefs.len(), 1);
    assert_eq!(
        parsed.initial_beliefs[0].schema_version.as_str(),
        EPISTEMIC_RECORD_SCHEMA_V1
    );
    assert!(String::from_utf8(first)
        .unwrap()
        .contains("initial_belief|belief_tomas_expected_coin|actor_tomas"));
}
