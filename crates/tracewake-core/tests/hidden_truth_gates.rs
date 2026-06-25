use std::collections::{BTreeMap, BTreeSet};

mod support;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{
    current_place_knowledge_context, current_place_perception_events, plan_local_actions,
    ActorDecisionTransaction, ActorDecisionTransactionInput, ActorDecisionTransactionOutcome,
    ActorKnownFact, LocalPlanRequest, NeedChangeCause, NeedKind, NeedState,
    NoHumanActorKnownSurfaceBuilder, NoHumanActorKnownSurfaceRequest, PlannerGoal, RoutineFamily,
    RoutineStep, SourceEventIds,
};
use tracewake_core::checksum::{compute_holder_known_context_hash, ChecksumContext};
use tracewake_core::debug_reports::item_location_report;
use tracewake_core::epistemics::{
    ActorKnownCarriedItemFact, ActorKnownContainerFact, ActorKnownCurrentPlaceFact,
    ActorKnownDoorFact, ActorKnownFoodSourceFact, ActorKnownItemFact, ActorKnownLocalActorFact,
    ActorKnownRouteFact, ActorKnownSleepAffordanceFact, ActorKnownWorkplaceFact, Channel,
    EpistemicProjection, KnowledgeContext, SourceRef, Stance,
};
use tracewake_core::events::apply::apply_epistemic_event;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, ContainerId, ContentManifestId, ContentVersion, ContradictionId,
    DoorId, EventId, FixtureId, FoodSupplyId, ItemId, ObservationId, PlaceId, ProcessId,
    SleepAffordanceId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::{
    build_embodied_view_model, EmbodiedPreflightSource, EmbodiedProjectionSource,
    EmbodiedTruthSnapshot,
};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, FoodSupplyState, PhysicalState, PlaceState,
};
use tracewake_core::time::SimTick;

const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const DEBUG_CAPABILITY_RS: &str = include_str!("../src/debug_capability.rs");
const KNOWLEDGE_CONTEXT_RS: &str = include_str!("../src/epistemics/knowledge_context.rs");
const EPISTEMIC_PROJECTION_RS: &str = include_str!("../src/epistemics/projection.rs");
const EPISTEMIC_BELIEF_RS: &str = include_str!("../src/epistemics/belief.rs");
const EPISTEMIC_OBSERVATION_RS: &str = include_str!("../src/epistemics/observation.rs");
const EPISTEMIC_CONTRADICTION_RS: &str = include_str!("../src/epistemics/contradiction.rs");
const VIEW_MODELS_RS: &str = include_str!("../src/view_models.rs");
const DEBUG_REPORTS_RS: &str = include_str!("../src/debug_reports.rs");
const CONTENT_SCHEMA_RS: &str = include_str!("../../tracewake-content/src/schema.rs");
const CONTENT_VALIDATE_RS: &str = include_str!("../../tracewake-content/src/validate.rs");
const CONTENT_SERIALIZATION_RS: &str = include_str!("../../tracewake-content/src/serialization.rs");
const CONTENT_LOAD_RS: &str = include_str!("../../tracewake-content/src/load.rs");
const TUI_INPUT_RS: &str = include_str!("../../tracewake-tui/src/input.rs");
const TUI_RENDER_RS: &str = include_str!("../../tracewake-tui/src/render.rs");
const TUI_DEBUG_PANELS_RS: &str = include_str!("../../tracewake-tui/src/debug_panels.rs");
const TUI_APP_RS: &str = include_str!("../../tracewake-tui/src/app.rs");
const TUI_TRANSCRIPT_RS: &str = include_str!("../../tracewake-tui/src/transcript.rs");

fn assert_source_lacks_any(source_name: &str, source: &str, banned: &[&str]) {
    let violations: Vec<_> = banned
        .iter()
        .copied()
        .filter(|token| source.contains(token))
        .collect();
    assert!(
        violations.is_empty(),
        "{} contains banned source token(s): {}",
        source_name,
        violations.join(", ")
    );
}

fn epistemic_guard_sources() -> [(&'static str, &'static str); 14] {
    [
        ("epistemics/knowledge_context.rs", KNOWLEDGE_CONTEXT_RS),
        ("epistemics/projection.rs", EPISTEMIC_PROJECTION_RS),
        ("epistemics/belief.rs", EPISTEMIC_BELIEF_RS),
        ("epistemics/observation.rs", EPISTEMIC_OBSERVATION_RS),
        ("epistemics/contradiction.rs", EPISTEMIC_CONTRADICTION_RS),
        ("view_models.rs", VIEW_MODELS_RS),
        ("debug_reports.rs", DEBUG_REPORTS_RS),
        ("content/schema.rs", CONTENT_SCHEMA_RS),
        ("content/validate.rs", CONTENT_VALIDATE_RS),
        ("content/serialization.rs", CONTENT_SERIALIZATION_RS),
        ("content/load.rs", CONTENT_LOAD_RS),
        ("tui/input.rs", TUI_INPUT_RS),
        ("tui/render.rs", TUI_RENDER_RS),
        ("tui/debug_panels.rs", TUI_DEBUG_PANELS_RS),
    ]
}

fn actor_id() -> ActorId {
    ActorId::new("actor_mara").unwrap()
}

fn place_id(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}

fn workplace_id(value: &str) -> WorkplaceId {
    WorkplaceId::new(value).unwrap()
}

fn container_id(value: &str) -> ContainerId {
    ContainerId::new(value).unwrap()
}

fn food_supply_id(value: &str) -> FoodSupplyId {
    FoodSupplyId::new(value).unwrap()
}

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
}

fn content_manifest_id() -> ContentManifestId {
    ContentManifestId::new("hidden_truth_gate_manifest").unwrap()
}

fn embodied_view(
    knowledge_context: &KnowledgeContext,
    world: &PhysicalState,
) -> tracewake_core::view_models::EmbodiedViewModel {
    let snapshot = EmbodiedTruthSnapshot::from_physical_state(knowledge_context, world);
    let projection_source =
        EmbodiedProjectionSource::from_sealed_context(knowledge_context, snapshot, None);
    let registry = registry();
    let content_manifest_id = content_manifest_id();
    let preflight_source = EmbodiedPreflightSource::new(world, &registry, &content_manifest_id);
    build_embodied_view_model(
        knowledge_context,
        &projection_source,
        &preflight_source,
        None,
    )
    .unwrap()
}

fn observed_current_place_context(world: &PhysicalState) -> KnowledgeContext {
    let manifest_id = content_manifest_id();
    let mut log = EventLog::new();
    let mut projection = EpistemicProjection::new(manifest_id.clone());
    let mut event_frontier = 0;
    for event in current_place_perception_events(world, &actor_id(), SimTick::ZERO, &manifest_id) {
        append_epistemic_event(&mut log, &mut projection, event);
        event_frontier += 1;
    }
    current_place_knowledge_context(
        world,
        Some(&projection),
        &actor_id(),
        SimTick::ZERO,
        &manifest_id,
        event_frontier,
    )
}

fn source_events(values: &[&str]) -> SourceEventIds {
    SourceEventIds::checked(
        values
            .iter()
            .map(|value| EventId::new(*value).unwrap())
            .collect(),
    )
    .unwrap()
}

fn fact_family_context() -> KnowledgeContext {
    KnowledgeContext::embodied_at_frontier_with_all_facts_and_observations(
        actor_id(),
        SimTick::new(7),
        42,
        vec![
            ActorKnownWorkplaceFact::new(
                workplace_id("workplace_mara"),
                place_id("market_square"),
                true,
                "evt.workplace.notice.1",
                source_events(&["event_workplace_notice_1"]),
                SimTick::new(5),
            ),
            ActorKnownWorkplaceFact::new(
                workplace_id("workplace_closed"),
                place_id("market_square"),
                false,
                "evt.workplace.notice.closed",
                source_events(&["event_workplace_notice_closed"]),
                SimTick::new(6),
            ),
        ],
        vec![ActorKnownCurrentPlaceFact::new(
            place_id("home_mara"),
            "Mara's home",
            "evt.current_place.1",
        )],
        vec![ActorKnownCarriedItemFact::new(
            ItemId::new("basket_mara").unwrap(),
            Location::CarriedBy(actor_id()),
            false,
            "evt.carried_item.1",
        )],
        vec![ActorKnownFoodSourceFact::with_believed_servings(
            food_supply_id("bread_pantry"),
            Some(2),
            "evt.food_source.1",
        )],
        vec![ActorKnownSleepAffordanceFact::new(
            SleepAffordanceId::new("bed_mara").unwrap(),
            place_id("home_mara"),
            "evt.sleep_place.1",
        )],
        vec![ActorKnownRouteFact::new(
            place_id("home_mara"),
            place_id("market_square"),
            "evt.route.1",
        )],
        vec![
            ActorKnownDoorFact::new(
                DoorId::new("door_home_market").unwrap(),
                place_id("home_mara"),
                place_id("market_square"),
                true,
                false,
                true,
                "evt.door.1",
            ),
            ActorKnownDoorFact::new(
                DoorId::new("door_home_lane").unwrap(),
                place_id("home_mara"),
                place_id("lane_mara"),
                true,
                false,
                false,
                "evt.door.open_nonblocking",
            ),
        ],
        vec![
            ActorKnownContainerFact::new(
                container_id("basket_mara"),
                false,
                true,
                "evt.container.1",
            ),
            ActorKnownContainerFact::new(
                container_id("open_box_mara"),
                true,
                false,
                "evt.container.open",
            ),
        ],
        vec![
            ActorKnownItemFact::new(
                ItemId::new("coin_mara").unwrap(),
                Location::InContainer(container_id("basket_mara")),
                false,
                "evt.item.1",
            ),
            ActorKnownItemFact::new(
                ItemId::new("needle_mara").unwrap(),
                Location::AtPlace(place_id("home_mara")),
                true,
                "evt.item.portable",
            ),
        ],
        vec![ActorKnownLocalActorFact::new(
            ActorId::new("actor_elena").unwrap(),
            "evt.local_actor.1",
        )],
    )
}

fn location_report_key(location: &Location) -> String {
    match location {
        Location::AtPlace(place_id) => format!("place:{}", place_id.as_str()),
        Location::InContainer(container_id) => format!("container:{}", container_id.as_str()),
        Location::CarriedBy(actor_id) => format!("carried:{}", actor_id.as_str()),
    }
}

fn knowledge_context_report_fingerprint(context: &KnowledgeContext) -> Vec<String> {
    let mut rows = vec![
        format!("context_id={}", context.holder_known_context_id().as_str()),
        format!(
            "context_hash={}",
            context.holder_known_context_hash().as_str()
        ),
        format!("mode={}", context.mode().stable_id()),
        format!("status={}", context.status().stable_id()),
        format!("frontier={}", context.event_frontier()),
        format!("debug={}", context.debug_non_diegetic()),
        format!("audit_passed={}", context.forbidden_truth_audit().passed()),
    ];
    rows.extend(
        context
            .allowed_sources()
            .iter()
            .map(|source| format!("allowed={}", source.stable_id())),
    );
    rows.extend(
        context
            .forbidden_sources()
            .iter()
            .map(|source| format!("forbidden={}", source.stable_id())),
    );
    rows.extend(context.provenance_entries().iter().map(|entry| {
        format!(
            "provenance={}:{}:{}",
            entry.kind().stable_id(),
            entry.source().stable_id(),
            entry.source_key()
        )
    }));
    rows.extend(context.actor_known_current_places().iter().map(|fact| {
        format!(
            "current_place={}:{}:{}",
            fact.place_id().as_str(),
            fact.display_label(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_carried_items().iter().map(|fact| {
        format!(
            "carried_item={}:source={}:portable={}:{}",
            fact.item_id().as_str(),
            location_report_key(fact.source()),
            fact.portable(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_workplaces().iter().map(|fact| {
        format!(
            "workplace={}@{}:open={}:events={}:tick={}:{}",
            fact.workplace_id().as_str(),
            fact.place_id().as_str(),
            fact.believed_access_open(),
            fact.source_event_ids()
                .as_slice()
                .iter()
                .map(EventId::as_str)
                .collect::<Vec<_>>()
                .join(","),
            fact.acquired_tick().value(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_sleep_affordances().iter().map(|fact| {
        format!(
            "sleep_affordance={}@{}:{}",
            fact.sleep_affordance_id().as_str(),
            fact.place_id().as_str(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_routes().iter().map(|fact| {
        format!(
            "route={}->{}:{}",
            fact.from_place_id().as_str(),
            fact.to_place_id().as_str(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_doors().iter().map(|fact| {
        format!(
            "door={}:{}->{}:open={}:locked={}:blocks={}:{}",
            fact.door_id().as_str(),
            fact.endpoint_a().as_str(),
            fact.endpoint_b().as_str(),
            fact.is_open(),
            fact.is_locked(),
            fact.blocks_movement_when_closed(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_containers().iter().map(|fact| {
        format!(
            "container={}:open={}:locked={}:{}",
            fact.container_id().as_str(),
            fact.is_open(),
            fact.is_locked(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_items().iter().map(|fact| {
        format!(
            "item={}:source={}:portable={}:{}",
            fact.item_id().as_str(),
            location_report_key(fact.source()),
            fact.portable(),
            fact.source_key()
        )
    }));
    rows.extend(context.actor_known_local_actors().iter().map(|fact| {
        format!(
            "local_actor={}:{}",
            fact.actor_id().as_str(),
            fact.source_key()
        )
    }));
    rows.sort();
    rows
}

fn helper_process_id(value: &str) -> ProcessId {
    ProcessId::new(value).unwrap()
}

fn ordering_key(tick: SimTick, action_id: &str) -> OrderingKey {
    OrderingKey::new(
        tick,
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(actor_id()),
        ProposalSequence::new(tick.value()),
        ActionId::new(action_id).unwrap(),
        vec![actor_id().as_str().to_string()],
        format!("{action_id}:{}", tick.value()),
    )
}

fn observation_event(event_id: &str, tick: SimTick) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(event_id).unwrap(),
        EventKind::ObservationRecorded,
        0,
        0,
        tick,
        ordering_key(tick, "observe"),
        content_manifest_id(),
        vec![EventCause::Process(helper_process_id(
            "process_hidden_truth_gate_observation",
        ))],
    )
    .unwrap();
    event.actor_id = Some(actor_id());
    event.participants = vec![actor_id().as_str().to_string()];
    event.place_id = Some(place_id("home_mara"));
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("observation_id", format!("obs_{event_id}")),
        PayloadField::new("observer_actor_id", actor_id().as_str()),
        PayloadField::new("channel", "direct_sight"),
        PayloadField::new("observed_tick", tick.value().to_string()),
        PayloadField::new("observer_place_id", "home_mara"),
        PayloadField::new("place_id", "home_mara"),
        PayloadField::new("confidence", "900"),
        PayloadField::new("source_event_id", event_id),
    ];
    event
}

fn food_observation_event(food_source_id: &str, tick: SimTick) -> EventEnvelope {
    let event_id = format!("event_visible_food_{}", food_source_id);
    let mut event = observation_event(&event_id, tick);
    event.payload.extend([
        PayloadField::new("perceived_kind", "visible_food_supply"),
        PayloadField::new("subject_id", "home_mara"),
        PayloadField::new("target_id", food_source_id),
        PayloadField::new("servings", "1"),
    ]);
    event
}

fn route_observation_event(from: &PlaceId, to: &PlaceId, tick: SimTick) -> EventEnvelope {
    let event_id = format!("event_visible_exit_{}_to_{}", from.as_str(), to.as_str());
    let mut event = observation_event(&event_id, tick);
    event.place_id = Some(from.clone());
    for field in &mut event.payload {
        if field.key == "observer_place_id" || field.key == "place_id" {
            field.value = from.as_str().to_string();
        }
    }
    event.payload.extend([
        PayloadField::new("perceived_kind", "visible_exit"),
        PayloadField::new("subject_id", from.as_str()),
        PayloadField::new("target_id", to.as_str()),
    ]);
    event
}

fn role_notice_event(
    workplace_id: &WorkplaceId,
    workplace_place_id: &PlaceId,
    tick: SimTick,
) -> EventEnvelope {
    let event_id = format!("event_notice_{}", workplace_id.as_str());
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(&event_id).unwrap(),
        EventKind::RoleAssignmentNoticeRecorded,
        0,
        0,
        tick,
        ordering_key(tick, "notice"),
        content_manifest_id(),
        vec![EventCause::Process(helper_process_id(
            "process_hidden_truth_gate_notice",
        ))],
    )
    .unwrap();
    event.actor_id = Some(actor_id());
    event.participants = vec![
        actor_id().as_str().to_string(),
        workplace_id.as_str().to_string(),
    ];
    event.place_id = Some(workplace_place_id.clone());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id().as_str()),
        PayloadField::new("workplace_id", workplace_id.as_str()),
        PayloadField::new("place_id", workplace_place_id.as_str()),
        PayloadField::new("access_open", "true"),
    ];
    event
}

fn belief_event(
    event_id: &str,
    belief_id: &str,
    tick: SimTick,
    source_event_id: &str,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(event_id).unwrap(),
        EventKind::BeliefUpdated,
        0,
        0,
        tick,
        ordering_key(tick, "belief"),
        content_manifest_id(),
        vec![EventCause::Event(EventId::new(source_event_id).unwrap())],
    )
    .unwrap();
    event.actor_id = Some(actor_id());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("belief_id", belief_id),
        PayloadField::new("holder_actor_id", actor_id().as_str()),
        PayloadField::new(
            "proposition",
            "item_located_in_container|food_visible_table|hidden_pantry",
        ),
        PayloadField::new("stance", "expects_true"),
        PayloadField::new("confidence", "0880"),
        PayloadField::new("source_event_id", source_event_id),
        PayloadField::new("acquired_tick", tick.value().to_string()),
        PayloadField::new("channel", "reading_placeholder_schema_only"),
    ];
    event
}

fn contradiction_event(
    event_id: &str,
    contradiction_id: &str,
    belief_id: &str,
    observation_id: &str,
    tick: SimTick,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(event_id).unwrap(),
        EventKind::ExpectationContradicted,
        0,
        0,
        tick,
        ordering_key(tick, "contradiction"),
        content_manifest_id(),
        vec![EventCause::Event(EventId::new(observation_id).unwrap())],
    )
    .unwrap();
    event.actor_id = Some(actor_id());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("contradiction_id", contradiction_id),
        PayloadField::new("holder_actor_id", actor_id().as_str()),
        PayloadField::new("prior_expectation_belief_id", belief_id),
        PayloadField::new("contradicting_observation_id", observation_id),
        PayloadField::new(
            "expected_proposition",
            "item_located_in_container|food_visible_table|hidden_pantry",
        ),
        PayloadField::new(
            "observed_proposition",
            "item_missing_from_expected_location|food_visible_table|in_container:hidden_pantry",
        ),
        PayloadField::new("detected_tick", tick.value().to_string()),
    ];
    event
}

fn append_epistemic_event(
    log: &mut EventLog,
    projection: &mut EpistemicProjection,
    event: EventEnvelope,
) -> EventId {
    let appended = log.append(event).unwrap();
    apply_epistemic_event(projection, &appended).unwrap();
    appended.event_id
}

fn agent_state(hunger: u16) -> AgentState {
    let mut state = AgentSeed::default();
    state.needs_by_actor_mut().insert(
        actor_id(),
        BTreeMap::from([
            (
                NeedKind::Hunger,
                NeedState::initial(
                    NeedKind::Hunger,
                    i32::from(hunger),
                    NeedChangeCause::FixtureInitial,
                ),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 100, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Safety,
                NeedState::initial(NeedKind::Safety, 100, NeedChangeCause::FixtureInitial),
            ),
        ]),
    );
    state.build()
}

fn context(
    known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    known_food_sources: BTreeSet<String>,
    known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
) -> tracewake_core::agent::ActorKnownPlanningState {
    let mut seeded_edges = BTreeMap::<PlaceId, BTreeSet<PlaceId>>::new();
    let mut seeded_food_sources = BTreeSet::<String>::new();
    let mut seeded_workplaces = BTreeMap::<WorkplaceId, PlaceId>::new();
    let mut log = EventLog::new();
    let mut projection = EpistemicProjection::new(content_manifest_id());
    let frame_event_id = append_epistemic_event(
        &mut log,
        &mut projection,
        observation_event("event_hidden_truth_gate_frame", SimTick::ZERO),
    );
    for (index, food_source_id) in known_food_sources.into_iter().enumerate() {
        append_epistemic_event(
            &mut log,
            &mut projection,
            food_observation_event(&food_source_id, SimTick::new(index as u64 + 1)),
        );
        seeded_food_sources.insert(food_source_id);
    }
    let mut route_index = 0_u64;
    for (from_place_id, to_place_ids) in known_edges {
        for to_place_id in to_place_ids {
            route_index += 1;
            append_epistemic_event(
                &mut log,
                &mut projection,
                route_observation_event(&from_place_id, &to_place_id, SimTick::new(route_index)),
            );
            seeded_edges
                .entry(from_place_id.clone())
                .or_default()
                .insert(to_place_id);
        }
    }
    for (workplace_id, workplace_place_id) in known_workplaces {
        append_epistemic_event(
            &mut log,
            &mut projection,
            role_notice_event(&workplace_id, &workplace_place_id, SimTick::new(1)),
        );
        seeded_workplaces.insert(workplace_id, workplace_place_id);
    }
    let agent_state = agent_state(880);
    let surface =
        NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
            projection: &projection,
            agent_state: &agent_state,
            actor_id: actor_id(),
            current_place_id: place_id("home_mara"),
            decision_tick: SimTick::ZERO,
            window_id: "hidden_truth_gate",
            window_end_tick: SimTick::new(4),
            current_place_witness_event_id: Some(frame_event_id.clone()),
            needs_witness_event_id: Some(frame_event_id.clone()),
            frame_event_id: Some(frame_event_id),
        })
        .build(&agent_state);
    let log_event_ids = log
        .events()
        .iter()
        .map(|event| event.event_id.clone())
        .collect::<BTreeSet<_>>();
    for fact in surface.context().actor_known_facts() {
        for source_event_id in fact.source_event_ids() {
            assert!(
                log_event_ids.contains(source_event_id),
                "actor-known fact {} cites source event {} outside the test log",
                fact.stable_id(),
                source_event_id.as_str()
            );
        }
    }
    assert_context_excludes_unseeded_hidden_counterparts(
        surface.context(),
        &seeded_edges,
        &seeded_food_sources,
        &seeded_workplaces,
    );
    surface.into_context()
}

fn assert_context_excludes_unseeded_hidden_counterparts(
    context: &tracewake_core::agent::ActorKnownPlanningState,
    seeded_edges: &BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    seeded_food_sources: &BTreeSet<String>,
    seeded_workplaces: &BTreeMap<WorkplaceId, PlaceId>,
) {
    for food_source_id in context.known_food_sources() {
        assert!(
            seeded_food_sources.contains(food_source_id),
            "unseeded food source {} leaked into context()",
            food_source_id
        );
    }
    for food_source_id in seeded_food_sources {
        assert!(
            context.known_food_sources().contains(food_source_id),
            "seeded food source {} is absent from context()",
            food_source_id
        );
    }

    for (from_place_id, to_place_ids) in context.known_edges() {
        for to_place_id in to_place_ids {
            assert!(
                seeded_edges
                    .get(from_place_id)
                    .is_some_and(|seeded_to| seeded_to.contains(to_place_id)),
                "unseeded route {}->{} leaked into context()",
                from_place_id.as_str(),
                to_place_id.as_str()
            );
        }
    }
    for (from_place_id, to_place_ids) in seeded_edges {
        for to_place_id in to_place_ids {
            assert!(
                context
                    .known_edges()
                    .get(from_place_id)
                    .is_some_and(|context_to| context_to.contains(to_place_id)),
                "seeded route {}->{} is absent from context()",
                from_place_id.as_str(),
                to_place_id.as_str()
            );
        }
    }

    for (workplace_id, place_id) in context.known_workplaces() {
        assert_eq!(
            seeded_workplaces.get(workplace_id),
            Some(place_id),
            "unseeded workplace {}@{} leaked into context()",
            workplace_id.as_str(),
            place_id.as_str()
        );
    }
    for (workplace_id, place_id) in seeded_workplaces {
        assert_eq!(
            context.known_workplaces().get(workplace_id),
            Some(place_id),
            "seeded workplace {}@{} is absent from context()",
            workplace_id.as_str(),
            place_id.as_str()
        );
    }
}

fn adversarial_truth_world() -> tracewake_core::state::PhysicalState {
    let mut world = PhysicalSeed::default();
    world.places_mut().insert(
        place_id("home_mara"),
        PlaceState::new(place_id("home_mara"), "Mara home"),
    );
    world.places_mut().insert(
        place_id("known_market"),
        PlaceState::new(place_id("known_market"), "Known market"),
    );
    world.places_mut().insert(
        place_id("hidden_workshop"),
        PlaceState::new(place_id("hidden_workshop"), "Hidden workshop"),
    );
    world.actors_mut().insert(
        actor_id(),
        ActorBody::new(actor_id(), place_id("home_mara")),
    );
    world.food_supplies_mut().insert(
        food_supply_id("food_visible_table"),
        FoodSupplyState::new(
            food_supply_id("food_visible_table"),
            Location::AtPlace(place_id("home_mara")),
            1,
            180,
        ),
    );
    let hidden_container_id = container_id("hidden_pantry");
    world.containers_mut().insert(
        hidden_container_id.clone(),
        ContainerState::fixed_at_place(hidden_container_id.clone(), place_id("home_mara")),
    );
    world.food_supplies_mut().insert(
        food_supply_id("food_hidden_pantry"),
        FoodSupplyState::new(
            food_supply_id("food_hidden_pantry"),
            Location::InContainer(hidden_container_id),
            1,
            220,
        ),
    );
    world.build()
}

fn planner_hidden_truth_fixture_witness_errors(
    world: &tracewake_core::state::PhysicalState,
    context: &tracewake_core::agent::ActorKnownPlanningState,
    known_food_source: &str,
    hidden_food_source: &str,
    known_route: (&PlaceId, &PlaceId),
    hidden_route: (&PlaceId, &PlaceId),
) -> Vec<String> {
    let mut errors = Vec::new();
    if !world
        .food_supplies()
        .contains_key(&food_supply_id(known_food_source))
    {
        errors.push(format!(
            "known food {known_food_source} is absent from truth"
        ));
    }
    if !world
        .food_supplies()
        .contains_key(&food_supply_id(hidden_food_source))
    {
        errors.push(format!(
            "hidden food {hidden_food_source} is absent from truth"
        ));
    }
    if !world.places().contains_key(known_route.0) || !world.places().contains_key(known_route.1) {
        errors.push(format!(
            "known route {}->{} is absent from truth places",
            known_route.0.as_str(),
            known_route.1.as_str()
        ));
    }
    if !world.places().contains_key(hidden_route.0) || !world.places().contains_key(hidden_route.1)
    {
        errors.push(format!(
            "hidden route {}->{} is absent from truth places",
            hidden_route.0.as_str(),
            hidden_route.1.as_str()
        ));
    }
    if !context.known_food_sources().contains(known_food_source) {
        errors.push(format!(
            "known food {known_food_source} is absent from actor-known context"
        ));
    }
    if context.known_food_sources().contains(hidden_food_source) {
        errors.push(format!(
            "hidden food {hidden_food_source} leaked into actor-known context"
        ));
    }
    if !context
        .known_edges()
        .get(known_route.0)
        .is_some_and(|edges| edges.contains(known_route.1))
    {
        errors.push(format!(
            "known route {}->{} is absent from actor-known context",
            known_route.0.as_str(),
            known_route.1.as_str()
        ));
    }
    if context
        .known_edges()
        .get(hidden_route.0)
        .is_some_and(|edges| edges.contains(hidden_route.1))
    {
        errors.push(format!(
            "hidden route {}->{} leaked into actor-known context",
            hidden_route.0.as_str(),
            hidden_route.1.as_str()
        ));
    }
    errors
}

fn proof_sources_are_actor_known(context: &tracewake_core::agent::ActorKnownPlanningState) {
    assert!(
        context.audit_with(&[]).actor_known_only,
        "fixture gate must assert provenance, not display text"
    );
    assert!(!context
        .proof_sources()
        .iter()
        .any(|source| source.contains("debug_omniscience") || source.contains("unproven")));
}

fn actor_known_cognition_fingerprint(
    context: &tracewake_core::agent::ActorKnownPlanningState,
) -> Vec<String> {
    let requests = [
        LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_visible_table".to_string()),
            budget: 2,
            actor_known_facts: Vec::new(),
        },
        LocalPlanRequest {
            routine_step: RoutineStep::MoveTowardPlace {
                action_id: "move".parse().unwrap(),
            },
            goal: PlannerGoal::ReachPlace(place_id("known_market")),
            budget: 2,
            actor_known_facts: Vec::new(),
        },
        LocalPlanRequest {
            routine_step: RoutineStep::StartWorkBlock {
                action_id: "work_block".parse().unwrap(),
            },
            goal: PlannerGoal::StartWorkBlock("workplace_visible_notice".to_string()),
            budget: 2,
            actor_known_facts: Vec::new(),
        },
    ];
    requests
        .into_iter()
        .map(|request| match plan_local_actions(context, &request) {
            Ok(success) => format!(
                "ok:{}:{}",
                success.proposals.len(),
                success
                    .proposals
                    .iter()
                    .map(|proposal| proposal.action_id.as_str().to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Err(failure) => format!("err:{}", failure.reason),
        })
        .collect()
}

#[test]
fn holder_known_fact_family_report_fingerprint_covers_keys_fields_and_audit() {
    let context = fact_family_context();
    let rows = knowledge_context_report_fingerprint(&context);
    let report_hash = compute_holder_known_context_hash(rows.clone()).hash;

    assert!(context.forbidden_truth_audit().passed());
    assert_eq!(
        context.holder_known_context_hash().as_str(),
        "hkc1-f1a06ad6077dcb76"
    );
    assert_eq!(report_hash.as_str(), "hkc1-8ad1ae1b01750844");
    for expected in [
        "mode=embodied",
        "status=current",
        "frontier=42",
        "debug=false",
        "audit_passed=true",
        "allowed=own_direct_observation",
        "allowed=own_search_or_touch_observation",
        "allowed=own_sound_observation",
        "allowed=own_absence_marker",
        "allowed=own_source_backed_belief",
        "forbidden=unobserved_event_log_truth",
        "forbidden=hidden_item_location",
        "forbidden=other_actors_private_beliefs",
        "forbidden=human_debug_notes",
        "forbidden=previous_possessed_actor_knowledge",
        "provenance=perception:own_direct_observation:context_source_policy",
        "provenance=observation:own_search_or_touch_observation:context_source_policy",
        "provenance=observation:own_sound_observation:context_source_policy",
        "provenance=record:own_absence_marker:context_source_policy",
        "provenance=belief:own_source_backed_belief:context_source_policy",
        "current_place=home_mara:Mara's home:evt.current_place.1",
        "carried_item=basket_mara:source=carried:actor_mara:portable=false:evt.carried_item.1",
        "workplace=workplace_mara@market_square:open=true:events=event_workplace_notice_1:tick=5:evt.workplace.notice.1",
        "workplace=workplace_closed@market_square:open=false:events=event_workplace_notice_closed:tick=6:evt.workplace.notice.closed",
        "sleep_affordance=bed_mara@home_mara:evt.sleep_place.1",
        "route=home_mara->market_square:evt.route.1",
        "door=door_home_market:home_mara->market_square:open=true:locked=false:blocks=true:evt.door.1",
        "door=door_home_lane:home_mara->lane_mara:open=true:locked=false:blocks=false:evt.door.open_nonblocking",
        "container=basket_mara:open=false:locked=true:evt.container.1",
        "container=open_box_mara:open=true:locked=false:evt.container.open",
        "item=coin_mara:source=container:basket_mara:portable=false:evt.item.1",
        "item=needle_mara:source=place:home_mara:portable=true:evt.item.portable",
        "local_actor=actor_elena:evt.local_actor.1",
    ] {
        assert!(rows.iter().any(|row| row == expected), "{expected}\n{rows:?}");
    }
}

#[test]
fn actor_known_context_unforgeable_from_truth() {
    assert!(
        ACTOR_KNOWN_RS.contains("pub(crate) fn from_observed_parts"),
        "actor-known planning context construction must stay crate-private"
    );
    assert!(
        !ACTOR_KNOWN_RS.contains("pub fn from_observed_parts"),
        "actor-known planning context construction must not become public"
    );
    assert!(
        !ACTOR_KNOWN_RS.contains("impl From<PhysicalState> for ActorKnownPlanningContext")
            && !ACTOR_KNOWN_RS.contains("impl From<&PhysicalState> for ActorKnownPlanningContext")
            && !ACTOR_KNOWN_RS.contains("from_physical_state"),
        "actor-known planning context must not gain a privileged raw-truth constructor"
    );
    assert!(
        ACTOR_KNOWN_RS.contains("```compile_fail")
            && ACTOR_KNOWN_RS.contains("ActorKnownPlanningContext::from_observed_parts")
            && ACTOR_KNOWN_RS.contains(
                "ActorKnownPlanningContext::from(PhysicalState::empty(NeedModelState::new(5, 3)))"
            ),
        "actor-known unforgeability must be documented by compile-fail examples"
    );
    assert!(
        DEBUG_CAPABILITY_RS.contains("pub(crate) const fn mint"),
        "debug capability minting must stay crate-private"
    );
    assert!(
        !DEBUG_CAPABILITY_RS.contains("pub const fn mint")
            && !DEBUG_CAPABILITY_RS.contains("pub fn mint"),
        "debug capability minting must not become public"
    );
    assert!(
        DEBUG_CAPABILITY_RS.contains("DebugCapability::mint()")
            && DEBUG_CAPABILITY_RS.contains("```compile_fail"),
        "debug capability minting must be covered by compile-fail documentation"
    );
}

#[test]
fn epistemic_context_projection_and_records_remain_sealed() {
    assert_source_lacks_any(
        "epistemics/knowledge_context.rs",
        KNOWLEDGE_CONTEXT_RS,
        &[
            "pub viewer_actor_id",
            "pub mode",
            "pub perception_scope",
            "pub belief_scope",
            "pub observation_scope",
            "pub debug_non_diegetic",
            "pub holder_known_context_hash",
            "pub forbidden_truth_audit",
        ],
    );
    assert!(
        KNOWLEDGE_CONTEXT_RS.contains("pub fn debug(")
            && KNOWLEDGE_CONTEXT_RS.contains("_capability: &DebugCapability"),
        "KnowledgeContext::debug must stay capability-gated if it remains public"
    );

    assert_source_lacks_any(
        "epistemics/projection.rs",
        EPISTEMIC_PROJECTION_RS,
        &[
            "pub observations_by_id",
            "pub beliefs_by_id",
            "pub beliefs_by_holder",
            "pub contradictions_by_id",
            "pub notebook_entries_by_actor",
            "pub fn insert_observation",
            "pub fn insert_belief",
            "pub fn insert_contradiction",
            "pub fn insert_notebook_entry",
        ],
    );

    assert_source_lacks_any(
        "epistemics/belief.rs",
        EPISTEMIC_BELIEF_RS,
        &[
            "pub belief_id:",
            "pub holder:",
            "pub proposition:",
            "pub stance:",
            "pub confidence:",
            "pub source:",
            "pub privacy_scope:",
            "pub schema_version:",
        ],
    );
    assert_source_lacks_any(
        "epistemics/observation.rs",
        EPISTEMIC_OBSERVATION_RS,
        &[
            "pub observation_id:",
            "pub observer_actor_id:",
            "pub channel:",
            "pub confidence:",
            "pub source:",
            "pub privacy_scope:",
            "pub schema_version:",
        ],
    );
    assert_source_lacks_any(
        "epistemics/contradiction.rs",
        EPISTEMIC_CONTRADICTION_RS,
        &[
            "pub contradiction_id:",
            "pub holder_actor_id:",
            "pub prior_expectation_belief_id:",
            "pub contradicting_observation_id:",
            "pub expected_proposition:",
            "pub observed_proposition:",
            "pub schema_version:",
        ],
    );
}

#[test]
fn epistemic_confidence_paths_do_not_use_raw_floats_or_hash_ordering() {
    let banned_tokens = [
        "f32",
        "f64",
        "parse::<f32>",
        "parse::<f64>",
        "HashMap",
        "HashSet",
    ];
    for (source_name, source) in epistemic_guard_sources().into_iter().chain([
        ("tui/app.rs", TUI_APP_RS),
        ("tui/transcript.rs", TUI_TRANSCRIPT_RS),
    ]) {
        assert_source_lacks_any(source_name, source, &banned_tokens);
    }

    let synthetic_float_leak = "let confidence: f32 = 0.5;";
    assert!(
        synthetic_float_leak.contains("f32"),
        "source guard self-coverage must include raw float confidence leaks"
    );
}

#[test]
fn hidden_food_closed_container_is_not_actor_known_food_source() {
    let known_route_from = place_id("home_mara");
    let known_route_to = place_id("known_market");
    let hidden_route_to = place_id("hidden_workshop");
    let context = context(
        BTreeMap::from([(
            known_route_from.clone(),
            BTreeSet::from([known_route_to.clone()]),
        )]),
        BTreeSet::from(["food_visible_table".to_string()]),
        BTreeMap::new(),
    );
    proof_sources_are_actor_known(&context);
    assert!(context.known_food_sources().contains("food_visible_table"));
    assert!(!context.known_food_sources().contains("food_hidden_pantry"));
    assert!(!context
        .known_edges()
        .get(&known_route_from)
        .is_some_and(|edges| edges.contains(&hidden_route_to)));

    let success = plan_local_actions(
        &context,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_visible_table".to_string()),
            budget: 3,
            actor_known_facts: Vec::new(),
        },
    )
    .expect("known visible food must remain planner-reachable");
    assert_eq!(success.proposals.len(), 1);
    assert_eq!(success.proposals[0].target_ids, vec!["food_visible_table"]);

    let failure = plan_local_actions(
        &context,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_hidden_pantry".to_string()),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(failure.reason, "food source is not actor-known");
    assert!(failure.trace.hidden_truth_audit_result.actor_known_only);
}

#[test]
fn embodied_affordances_exclude_hidden_food_in_closed_container() {
    let mut world = PhysicalSeed::default();
    world.places_mut().insert(
        place_id("home_mara"),
        PlaceState::new(place_id("home_mara"), "Mara home"),
    );
    world.actors_mut().insert(
        actor_id(),
        ActorBody::new(actor_id(), place_id("home_mara")),
    );

    world.food_supplies_mut().insert(
        food_supply_id("food_empty_pantry_mara"),
        FoodSupplyState::new(
            food_supply_id("food_empty_pantry_mara"),
            Location::AtPlace(place_id("home_mara")),
            0,
            180,
        ),
    );
    let hidden_container_id = container_id("hidden_pantry");
    world.containers_mut().insert(
        hidden_container_id.clone(),
        ContainerState::fixed_at_place(hidden_container_id.clone(), place_id("home_mara")),
    );
    world.food_supplies_mut().insert(
        food_supply_id("food_hidden_pantry"),
        FoodSupplyState::new(
            food_supply_id("food_hidden_pantry"),
            Location::InContainer(hidden_container_id),
            1,
            220,
        ),
    );
    let world = world.build();

    let knowledge_context = observed_current_place_context(&world);
    let view = embodied_view(&knowledge_context, &world);

    assert!(view
        .semantic_actions
        .iter()
        .any(|entry| { entry.semantic_action_id.as_str() == "eat.food.food_empty_pantry_mara" }));
    assert!(!view.semantic_actions.iter().any(|entry| {
        entry
            .semantic_action_id
            .as_str()
            .contains("food_hidden_pantry")
            || entry.label.contains("food_hidden_pantry")
            || entry
                .target_ids
                .iter()
                .any(|target| target == "food_hidden_pantry")
            || entry
                .availability
                .actor_safe_summary()
                .is_some_and(|why| why.contains("food_hidden_pantry"))
    }));
}

#[test]
fn hidden_food_unknown_route_does_not_become_transaction_target() {
    let agent_state = agent_state(920);
    let context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
        actor_id: actor_id(),
        decision_tick: SimTick::ZERO,
        agent_state: &agent_state,
        actor_known_context: &context,
        source_event_ids: None,
        source_event_kinds: None,
        routine_window_family: Some(RoutineFamily::EatMeal),
        include_idle_fallback: true,
    });
    match outcome {
        ActorDecisionTransactionOutcome::Proposed(proposed) => {
            assert_ne!(proposed.proposal.action_id().as_str(), "eat");
            assert!(!proposed
                .proposal
                .target_ids()
                .iter()
                .any(|target| target == "food_hidden_pantry"));
            assert!(
                proposed
                    .decision_trace_record
                    .hidden_truth_audit_result
                    .actor_known_only
            );
        }
        ActorDecisionTransactionOutcome::Stuck { diagnostic } => {
            assert!(!format!("{diagnostic:?}").contains("food_hidden_pantry"));
        }
    }
}

#[test]
fn workplace_requires_assignment_or_observation_provenance() {
    let hidden_workplace = workplace_id("workplace_hidden");
    let hidden_place = place_id("hidden_workshop");
    let context_without_workplace = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    assert!(!context_without_workplace
        .known_workplaces()
        .contains_key(&hidden_workplace));

    let failure = plan_local_actions(
        &context_without_workplace,
        &LocalPlanRequest {
            routine_step: RoutineStep::StartWorkBlock {
                action_id: "work_block".parse().unwrap(),
            },
            goal: PlannerGoal::StartWorkBlock(hidden_workplace.as_str().to_string()),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(
        failure.reason,
        "workplace is not actor-known at current place"
    );

    let context_with_assignment = context(
        BTreeMap::new(),
        BTreeSet::new(),
        BTreeMap::from([(hidden_workplace.clone(), hidden_place)]),
    );
    assert!(context_with_assignment
        .proof_sources()
        .iter()
        .any(|source| source.contains("routine_assignment")));
}

#[test]
fn hidden_route_edge_absent_from_actor_context_blocks_route_plan() {
    let known_route_from = place_id("home_mara");
    let known_route_to = place_id("known_market");
    let context = context(
        BTreeMap::from([(
            known_route_from.clone(),
            BTreeSet::from([known_route_to.clone()]),
        )]),
        BTreeSet::from(["food_visible_table".to_string()]),
        BTreeMap::new(),
    );
    proof_sources_are_actor_known(&context);
    assert!(context
        .known_edges()
        .get(&known_route_from)
        .is_some_and(|edges| edges.contains(&known_route_to)));
    assert!(!context
        .known_edges()
        .get(&place_id("home_mara"))
        .is_some_and(|edges| edges.contains(&place_id("hidden_workshop"))));

    let success = plan_local_actions(
        &context,
        &LocalPlanRequest {
            routine_step: RoutineStep::MoveTowardPlace {
                action_id: "move".parse().unwrap(),
            },
            goal: PlannerGoal::ReachPlace(known_route_to.clone()),
            budget: 2,
            actor_known_facts: Vec::new(),
        },
    )
    .expect("known visible route must remain planner-reachable");
    assert_eq!(success.proposals.len(), 1);
    assert_eq!(
        success.proposals[0].target_ids,
        vec![known_route_to.as_str()]
    );

    let failure = plan_local_actions(
        &context,
        &LocalPlanRequest {
            routine_step: RoutineStep::MoveTowardPlace {
                action_id: "move".parse().unwrap(),
            },
            goal: PlannerGoal::ReachPlace(place_id("hidden_workshop")),
            budget: 3,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(failure.reason, "no actor-known route to target");
}

#[test]
fn hidden_truth_metamorphism_and_provenance_deletion_are_relational() {
    let known_route_from = place_id("home_mara");
    let known_route_to = place_id("known_market");
    let known_workplace = workplace_id("workplace_visible_notice");
    let context_with_all_provenance = context(
        BTreeMap::from([(
            known_route_from.clone(),
            BTreeSet::from([known_route_to.clone()]),
        )]),
        BTreeSet::from(["food_visible_table".to_string()]),
        BTreeMap::from([(known_workplace.clone(), known_route_to.clone())]),
    );

    let baseline_cognition = actor_known_cognition_fingerprint(&context_with_all_provenance);
    let base_world = adversarial_truth_world();
    let mut mutated_truth = PhysicalSeed::from_state(&base_world);
    mutated_truth.food_supplies_mut().insert(
        food_supply_id("food_unobserved_variant"),
        FoodSupplyState::new(
            food_supply_id("food_unobserved_variant"),
            Location::AtPlace(place_id("hidden_workshop")),
            7,
            300,
        ),
    );
    let mutated_truth = mutated_truth.build();
    for world in [&base_world, &mutated_truth] {
        let errors = planner_hidden_truth_fixture_witness_errors(
            world,
            &context_with_all_provenance,
            "food_visible_table",
            "food_hidden_pantry",
            (&known_route_from, &known_route_to),
            (&known_route_from, &place_id("hidden_workshop")),
        );
        assert!(
            errors.is_empty(),
            "hidden-truth fixture witness must stay satisfied: {errors:#?}"
        );
        assert_eq!(
            actor_known_cognition_fingerprint(&context_with_all_provenance),
            baseline_cognition,
            "unobserved truth changes must not alter actor-known planner output"
        );
    }

    let without_food_source = context(
        BTreeMap::from([(
            known_route_from.clone(),
            BTreeSet::from([known_route_to.clone()]),
        )]),
        BTreeSet::new(),
        BTreeMap::from([(known_workplace.clone(), known_route_to.clone())]),
    );
    let food_failure = plan_local_actions(
        &without_food_source,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_visible_table".to_string()),
            budget: 2,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(food_failure.reason, "food source is not actor-known");

    let without_route_source = context(
        BTreeMap::new(),
        BTreeSet::from(["food_visible_table".to_string()]),
        BTreeMap::from([(known_workplace.clone(), known_route_to.clone())]),
    );
    let route_failure = plan_local_actions(
        &without_route_source,
        &LocalPlanRequest {
            routine_step: RoutineStep::MoveTowardPlace {
                action_id: "move".parse().unwrap(),
            },
            goal: PlannerGoal::ReachPlace(known_route_to.clone()),
            budget: 2,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(route_failure.reason, "no actor-known route to target");

    let wrong_kind_substitution = context(
        BTreeMap::from([(known_route_from, BTreeSet::from([known_route_to.clone()]))]),
        BTreeSet::from(["food_visible_table".to_string()]),
        BTreeMap::new(),
    );
    let workplace_failure = plan_local_actions(
        &wrong_kind_substitution,
        &LocalPlanRequest {
            routine_step: RoutineStep::StartWorkBlock {
                action_id: "work_block".parse().unwrap(),
            },
            goal: PlannerGoal::StartWorkBlock(known_workplace.as_str().to_string()),
            budget: 2,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(
        workplace_failure.reason,
        "workplace is not actor-known at current place"
    );
}

#[test]
fn context_rejects_hidden_counterpart_injection() {
    let synthetic_context_hidden_food_injection = std::panic::catch_unwind(|| {
        let mut log = EventLog::new();
        let mut projection = EpistemicProjection::new(content_manifest_id());
        let frame_event_id = append_epistemic_event(
            &mut log,
            &mut projection,
            observation_event("event_hidden_truth_gate_frame", SimTick::ZERO),
        );
        append_epistemic_event(
            &mut log,
            &mut projection,
            food_observation_event("food_hidden_pantry", SimTick::new(1)),
        );
        let agent_state = agent_state(880);
        let surface =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &projection,
                agent_state: &agent_state,
                actor_id: actor_id(),
                current_place_id: place_id("home_mara"),
                decision_tick: SimTick::ZERO,
                window_id: "hidden_truth_gate",
                window_end_tick: SimTick::new(4),
                current_place_witness_event_id: Some(frame_event_id.clone()),
                needs_witness_event_id: Some(frame_event_id.clone()),
                frame_event_id: Some(frame_event_id),
            })
            .build(&agent_state);
        assert_context_excludes_unseeded_hidden_counterparts(
            surface.context(),
            &BTreeMap::new(),
            &BTreeSet::new(),
            &BTreeMap::new(),
        );
    });
    assert!(
        synthetic_context_hidden_food_injection.is_err(),
        "synthetic_context_hidden_food_injection must fail inside context()"
    );

    let synthetic_context_hidden_route_injection = std::panic::catch_unwind(|| {
        let mut log = EventLog::new();
        let mut projection = EpistemicProjection::new(content_manifest_id());
        let frame_event_id = append_epistemic_event(
            &mut log,
            &mut projection,
            observation_event("event_hidden_truth_gate_frame", SimTick::ZERO),
        );
        append_epistemic_event(
            &mut log,
            &mut projection,
            route_observation_event(
                &place_id("home_mara"),
                &place_id("hidden_workshop"),
                SimTick::new(1),
            ),
        );
        let agent_state = agent_state(880);
        let surface =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &projection,
                agent_state: &agent_state,
                actor_id: actor_id(),
                current_place_id: place_id("home_mara"),
                decision_tick: SimTick::ZERO,
                window_id: "hidden_truth_gate",
                window_end_tick: SimTick::new(4),
                current_place_witness_event_id: Some(frame_event_id.clone()),
                needs_witness_event_id: Some(frame_event_id.clone()),
                frame_event_id: Some(frame_event_id),
            })
            .build(&agent_state);
        assert_context_excludes_unseeded_hidden_counterparts(
            surface.context(),
            &BTreeMap::new(),
            &BTreeSet::new(),
            &BTreeMap::new(),
        );
    });
    assert!(
        synthetic_context_hidden_route_injection.is_err(),
        "synthetic hidden-route injection must fail inside context()"
    );
}

#[test]
fn planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context() {
    let world = adversarial_truth_world();
    let known_route_from = place_id("home_mara");
    let known_route_to = place_id("known_market");
    let hidden_route_to = place_id("hidden_workshop");
    let empty_context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    let errors = planner_hidden_truth_fixture_witness_errors(
        &world,
        &empty_context,
        "food_visible_table",
        "food_hidden_pantry",
        (&known_route_from, &known_route_to),
        (&known_route_from, &hidden_route_to),
    );
    assert!(
        errors
            .iter()
            .any(|error| error.contains("known food food_visible_table")),
        "empty adversarial food fixture must fail the witness check: {errors:#?}"
    );
    assert!(
        errors
            .iter()
            .any(|error| error.contains("known route home_mara->known_market")),
        "empty adversarial route fixture must fail the witness check: {errors:#?}"
    );
}

#[test]
fn debug_omniscience_facts_are_excluded_from_planner_context() {
    let context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    proof_sources_are_actor_known(&context);
    assert!(!context
        .proof_sources()
        .iter()
        .any(|source| source.contains("debug_omniscience")));

    let unproven = ActorKnownFact::unproven(
        "debug_hidden_food",
        "debug-only omniscience must not be accepted as actor-known",
    );
    assert!(!context.audit_with(&[unproven]).actor_known_only);
}

#[test]
fn debug_truth_never_enters_holder_known_context_hash() {
    let mut world = PhysicalSeed::default();
    world.places_mut().insert(
        place_id("home_mara"),
        PlaceState::new(place_id("home_mara"), "Mara home"),
    );
    world.actors_mut().insert(
        actor_id(),
        ActorBody::new(actor_id(), place_id("home_mara")),
    );
    world.food_supplies_mut().insert(
        food_supply_id("food_empty_pantry_mara"),
        FoodSupplyState::new(
            food_supply_id("food_empty_pantry_mara"),
            Location::AtPlace(place_id("home_mara")),
            0,
            180,
        ),
    );
    let hidden_container_id = container_id("hidden_pantry");
    world.containers_mut().insert(
        hidden_container_id.clone(),
        ContainerState::fixed_at_place(hidden_container_id.clone(), place_id("home_mara")),
    );
    world.food_supplies_mut().insert(
        food_supply_id("food_hidden_pantry"),
        FoodSupplyState::new(
            food_supply_id("food_hidden_pantry"),
            Location::InContainer(hidden_container_id),
            1,
            220,
        ),
    );
    let world = world.build();
    let knowledge_context = observed_current_place_context(&world);
    let before_view = embodied_view(&knowledge_context, &world);

    let debug_report = item_location_report(
        &world,
        &tracewake_core::events::log::EventLog::new(),
        &ItemId::new("food_hidden_pantry").unwrap(),
        &ChecksumContext {
            fixture_id: FixtureId::new("debug_truth_hash_gate").unwrap(),
            content_version: ContentVersion::new("hidden_truth_gate_manifest").unwrap(),
            sim_tick: SimTick::ZERO,
            world_stream_position_applied: 0,
        },
    );
    assert!(debug_report.debug_only());
    assert!(format!("{debug_report:?}").contains("food_hidden_pantry"));

    let after_view = embodied_view(&knowledge_context, &world);

    assert_eq!(
        after_view.holder_known_context_hash(),
        before_view.holder_known_context_hash()
    );
    assert_eq!(
        after_view.holder_known_context_source_summary(),
        before_view.holder_known_context_source_summary()
    );
    assert!(!after_view
        .holder_known_context_source_summary()
        .contains("debug"));
    assert!(!after_view.semantic_actions.iter().any(|entry| entry
        .target_ids
        .iter()
        .any(|target| target == "food_hidden_pantry")));
}

#[test]
fn embodied_place_label_is_captured_before_truth_preflight() {
    let mut seed = PhysicalSeed::default();
    seed.places_mut().insert(
        place_id("home_mara"),
        PlaceState::new(place_id("home_mara"), "Mara home"),
    );
    seed.actors_mut().insert(
        actor_id(),
        ActorBody::new(actor_id(), place_id("home_mara")),
    );
    let original_world = seed.build();
    let knowledge_context = observed_current_place_context(&original_world);
    let snapshot = EmbodiedTruthSnapshot::from_physical_state(&knowledge_context, &original_world);
    let projection_source =
        EmbodiedProjectionSource::from_sealed_context(&knowledge_context, snapshot, None);

    let mut mutated_seed = PhysicalSeed::from_state(&original_world);
    mutated_seed.places_mut().insert(
        place_id("home_mara"),
        PlaceState::new(place_id("home_mara"), "Mutated truth label"),
    );
    let mutated_world = mutated_seed.build();
    let registry = registry();
    let content_manifest_id = content_manifest_id();
    let preflight_source =
        EmbodiedPreflightSource::new(&mutated_world, &registry, &content_manifest_id);

    let view = build_embodied_view_model(
        &knowledge_context,
        &projection_source,
        &preflight_source,
        None,
    )
    .unwrap();

    assert_eq!(view.place_label, "Mara home");
    assert_ne!(view.place_label, "Mutated truth label");
}

#[test]
fn actor_known_local_actor_reaches_embodied_view_model_with_context_provenance() {
    let known_actor = ActorId::new("actor_elias").unwrap();
    let hidden_actor = ActorId::new("actor_hidden_neighbor").unwrap();
    let mut seed = PhysicalSeed::default();
    seed.places_mut().insert(
        place_id("home_mara"),
        PlaceState::new(place_id("home_mara"), "Mara home"),
    );
    seed.actors_mut().insert(
        actor_id(),
        ActorBody::new(actor_id(), place_id("home_mara")),
    );
    seed.actors_mut().insert(
        known_actor.clone(),
        ActorBody::new(known_actor.clone(), place_id("home_mara")),
    );
    seed.actors_mut().insert(
        hidden_actor.clone(),
        ActorBody::new(hidden_actor.clone(), place_id("home_mara")),
    );
    let world = seed.build();
    let context = KnowledgeContext::embodied_at_frontier_with_all_facts_and_observations(
        actor_id(),
        SimTick::new(3),
        7,
        Vec::new(),
        vec![ActorKnownCurrentPlaceFact::new(
            place_id("home_mara"),
            "Mara home",
            "event_visible_place_home_mara",
        )],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![ActorKnownLocalActorFact::new(
            known_actor.clone(),
            "event_visible_actor_elias",
        )],
    );

    assert_eq!(context.event_frontier(), 7);
    assert!(context.actor_known_local_actors().iter().any(|fact| {
        fact.actor_id() == &known_actor && fact.source_key() == "event_visible_actor_elias"
    }));

    let view = embodied_view(&context, &world);

    assert!(view
        .local_actors
        .iter()
        .any(|actor| actor.actor_id == known_actor));
    assert!(!view
        .local_actors
        .iter()
        .any(|actor| actor.actor_id == hidden_actor));
    assert_eq!(
        view.holder_known_context_id(),
        context.holder_known_context_id()
    );
    assert_eq!(
        view.holder_known_context_hash(),
        context.holder_known_context_hash()
    );
    assert_eq!(
        view.holder_known_context_frontier(),
        context.event_frontier()
    );
    assert!(!view.debug_available());
    assert!(!view.holder_known_context_source_summary().contains("debug"));
}

#[test]
fn epistemic_event_fields_survive_into_sealed_context_and_replay() {
    let mut log = EventLog::new();
    let mut projection = EpistemicProjection::new(content_manifest_id());
    let observation_id = "obs_event_hidden_truth_survival_observation";
    let observation = observation_event("event_hidden_truth_survival_observation", SimTick::new(5));
    append_epistemic_event(&mut log, &mut projection, observation.clone());
    let belief = belief_event(
        "event_hidden_truth_survival_belief",
        "belief_hidden_truth_survival",
        SimTick::new(6),
        "event_hidden_truth_survival_observation",
    );
    append_epistemic_event(&mut log, &mut projection, belief);
    let contradiction = contradiction_event(
        "event_hidden_truth_survival_contradiction",
        "contradiction_hidden_truth_survival",
        "belief_hidden_truth_survival",
        observation_id,
        SimTick::new(7),
    );
    append_epistemic_event(&mut log, &mut projection, contradiction);

    let context = KnowledgeContext::embodied(actor_id(), SimTick::new(8));
    let belief = projection
        .beliefs_for_context(&context)
        .into_iter()
        .find(|belief| {
            belief.belief_id() == &BeliefId::new("belief_hidden_truth_survival").unwrap()
        })
        .expect("belief event applied");
    assert_eq!(belief.stance(), Stance::ExpectsTrue);
    assert_eq!(
        belief.channel(),
        Some(Channel::ReadingPlaceholderSchemaOnly)
    );
    assert_eq!(
        belief.source(),
        &SourceRef::Event(EventId::new("event_hidden_truth_survival_observation").unwrap())
    );

    let observation = projection
        .observations_for_context(&context)
        .into_iter()
        .find(|observation| {
            observation.observation_id() == &ObservationId::new(observation_id).unwrap()
        })
        .expect("observation event applied");
    assert_eq!(observation.channel(), Channel::DirectSight);
    assert_eq!(
        observation.source(),
        &SourceRef::Event(EventId::new("event_hidden_truth_survival_observation").unwrap())
    );

    let contradiction = projection
        .contradictions_for_context(&context)
        .into_iter()
        .find(|contradiction| {
            contradiction.contradiction_id()
                == &ContradictionId::new("contradiction_hidden_truth_survival").unwrap()
        })
        .expect("contradiction event applied");
    assert_eq!(
        contradiction.prior_expectation_belief_id(),
        &BeliefId::new("belief_hidden_truth_survival").unwrap()
    );
    assert_eq!(
        contradiction.contradicting_observation_id(),
        &ObservationId::new(observation_id).unwrap()
    );

    let replayed_log = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
    let mut replayed_projection = EpistemicProjection::new(content_manifest_id());
    for event in replayed_log.events() {
        apply_epistemic_event(&mut replayed_projection, event).unwrap();
    }
    assert_eq!(
        replayed_projection.compute_checksum().checksum,
        projection.compute_checksum().checksum
    );
}
