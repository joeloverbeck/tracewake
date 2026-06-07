use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ContentManifestId, EventId, FoodSupplyId};
use crate::location::Location;
use crate::scheduler::OrderingKey;
use crate::state::{FoodSupplyState, PhysicalState};

pub fn build_eat_events(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<Vec<EventEnvelope>, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    let actor = state.actors.get(&actor_id).ok_or_else(actor_missing)?;
    let food_supply_id = food_supply_target(proposal)?;

    let Some(food) = state.food_supplies.get(&food_supply_id) else {
        return Ok(vec![eat_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            food_supply_id.as_str(),
            "knowledge",
            "food source absent",
            "target food supply is not present in physical state",
        )]);
    };

    if food.is_empty() {
        return Ok(vec![eat_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            food_supply_id.as_str(),
            "resource",
            "food source empty",
            "target food supply had zero servings",
        )]);
    }

    if let Some(reason) = access_failure(state, food, &actor.actor_id, &actor.current_place_id) {
        return Ok(vec![eat_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            food_supply_id.as_str(),
            "access",
            reason,
            reason,
        )]);
    }

    let consumed = food_consumed_event(
        proposal,
        ordering_key,
        content_manifest_id,
        food,
        food.servings - 1,
    );
    let need_delta = hunger_delta_event(
        proposal,
        ordering_key,
        content_manifest_id,
        &consumed,
        -food.hunger_reduction_per_serving,
    );
    Ok(vec![consumed, need_delta])
}

fn food_consumed_event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    food: &FoodSupplyState,
    servings_after: u32,
) -> EventEnvelope {
    let actor_id = proposal.actor_id.as_ref().expect("actor checked");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.food_consumed.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::FoodConsumed,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Proposal(proposal.proposal_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![actor_id.to_string(), food.food_supply_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("food_supply_id", food.food_supply_id.as_str()),
        PayloadField::new("servings_consumed", "1"),
        PayloadField::new("servings_before", food.servings.to_string()),
        PayloadField::new("servings_after", servings_after.to_string()),
        PayloadField::new(
            "hunger_delta",
            (-food.hunger_reduction_per_serving).to_string(),
        ),
        PayloadField::new("location", location_payload(&food.location)),
    ];
    event.effects_summary = "food serving consumed".to_string();
    event
}

fn hunger_delta_event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    consumed_event: &EventEnvelope,
    delta: i32,
) -> EventEnvelope {
    let actor_id = proposal.actor_id.as_ref().expect("actor checked");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.eat_hunger_delta.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::NeedDeltaApplied,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Event(consumed_event.event_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![actor_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("need_kind", "hunger"),
        PayloadField::new("delta", delta.to_string()),
        PayloadField::new("cause_kind", "action_effect"),
        PayloadField::new("cause_action_id", "eat"),
    ];
    event.effects_summary = "hunger reduced by modeled food consumption".to_string();
    event
}

fn eat_failed_event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    food_supply_id: &str,
    blocker_kind: &str,
    reason: &str,
    debug_reason: &str,
) -> EventEnvelope {
    let actor_id = proposal.actor_id.as_ref().expect("actor checked");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.eat_failed.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::EatFailed,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Proposal(proposal.proposal_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![actor_id.to_string(), food_supply_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("food_supply_id", food_supply_id),
        PayloadField::new("blocker_kind", blocker_kind),
        PayloadField::new("reason", reason),
        PayloadField::new("observation_ancestry", "proposal_target_validation"),
        PayloadField::new("absence_ancestry", debug_reason),
    ];
    event.effects_summary = format!("eat failed: {reason}");
    event
}

fn access_failure<'a>(
    state: &'a PhysicalState,
    food: &FoodSupplyState,
    actor_id: &crate::ids::ActorId,
    actor_place_id: &crate::ids::PlaceId,
) -> Option<&'a str> {
    match &food.location {
        Location::AtPlace(place_id) if place_id == actor_place_id => None,
        Location::AtPlace(_) => Some("food source not at actor place"),
        Location::CarriedBy(carrier_id) if carrier_id == actor_id => None,
        Location::CarriedBy(carrier_id) => Some(if state.actors.contains_key(carrier_id) {
            "food source carried by another actor"
        } else {
            "food source carrier absent"
        }),
        Location::InContainer(container_id) => {
            let Some(container) = state.containers.get(container_id) else {
                return Some("food source container absent");
            };
            if container.location != Location::AtPlace(actor_place_id.clone()) {
                return Some("food source container not reachable");
            }
            if !container.is_open {
                return Some("food source container closed");
            }
            None
        }
    }
}

fn food_supply_target(proposal: &Proposal) -> Result<FoodSupplyId, ActionRejection> {
    let target = proposal.target_ids.first().ok_or_else(|| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            "target_count",
            &proposal.target_ids.len().to_string(),
            "There is no food source to eat from.",
            "eat proposal did not include a food supply target",
        )
    })?;
    FoodSupplyId::new(target).map_err(|_| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::UnsupportedTargetKind,
            "food_supply_id",
            target,
            "That food source is invalid.",
            "eat target was not a stable food supply ID",
        )
    })
}

fn location_payload(location: &Location) -> String {
    match location {
        Location::AtPlace(id) => format!("place:{}", id.as_str()),
        Location::InContainer(id) => format!("container:{}", id.as_str()),
        Location::CarriedBy(id) => format!("actor:{}", id.as_str()),
    }
}

fn actor_missing() -> ActionRejection {
    ActionRejection::new(
        PipelineStage::ActorLookup,
        ReasonCode::ActorNotFound,
        Vec::new(),
        "That actor cannot eat.",
        "actor was missing",
    )
}

fn reject(
    failed_stage: PipelineStage,
    reason_code: ReasonCode,
    key: &'static str,
    value: &str,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> ActionRejection {
    ActionRejection::new(
        failed_stage,
        reason_code,
        vec![CheckedFact::new(key, value)],
        actor_visible_summary,
        debug_summary,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::registry::ActionRegistry;
    use crate::actions::report::ReportStatus;
    use crate::events::apply::apply_event;
    use crate::events::log::EventLog;
    use crate::ids::{ActionId, ActorId, ContainerId, PlaceId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, ContainerState, PlaceState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn food_supply_id() -> FoodSupplyId {
        FoodSupplyId::new("food_soup_pot").unwrap()
    }

    fn place_id() -> PlaceId {
        PlaceId::new("kitchen").unwrap()
    }

    fn state(servings: u32) -> PhysicalState {
        let mut state = PhysicalState::default();
        state
            .places
            .insert(place_id(), PlaceState::new(place_id(), "Kitchen"));
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), place_id()));
        state.food_supplies.insert(
            food_supply_id(),
            FoodSupplyState::new(
                food_supply_id(),
                Location::AtPlace(place_id()),
                servings,
                120,
            ),
        );
        state
    }

    fn proposal() -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_eat").unwrap(),
            crate::actions::proposal::ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("eat").unwrap(),
            SimTick::new(4),
        );
        proposal.target_ids.push(food_supply_id().to_string());
        proposal
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(4),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new("eat").unwrap(),
            vec![food_supply_id().to_string()],
            "eat",
        )
    }

    #[test]
    fn eat_consumes_one_serving_and_emits_hunger_delta() {
        let mut state = state(2);
        let events = build_eat_events(
            &state,
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(events[0].event_type, EventKind::FoodConsumed);
        assert_eq!(events[1].event_type, EventKind::NeedDeltaApplied);
        assert!(events[1]
            .payload
            .iter()
            .any(|field| field.key == "delta" && field.value == "-120"));
        apply_event(&mut state, &events[0]).unwrap();
        assert_eq!(state.food_supplies[&food_supply_id()].servings, 1);
    }

    #[test]
    fn eat_empty_food_source_fails_without_consumption() {
        let state = state(0);
        let events = build_eat_events(
            &state,
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventKind::EatFailed);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "blocker_kind" && field.value == "resource"));
    }

    #[test]
    fn eat_in_closed_container_fails_with_access_blocker() {
        let mut state = state(1);
        let container_id = ContainerId::new("pantry").unwrap();
        let mut container = ContainerState::fixed_at_place(container_id.clone(), place_id());
        container.is_open = false;
        state.containers.insert(container_id.clone(), container);
        state
            .food_supplies
            .get_mut(&food_supply_id())
            .unwrap()
            .location = Location::InContainer(container_id);

        let events = build_eat_events(
            &state,
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(events[0].event_type, EventKind::EatFailed);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "blocker_kind" && field.value == "access"));
        assert_eq!(state.food_supplies[&food_supply_id()].servings, 1);
    }

    #[test]
    fn eat_runs_through_shared_pipeline_and_decrements_servings() {
        let mut state = state(2);
        let mut log = EventLog::new();
        let mut agent_state = crate::state::AgentState::default();
        agent_state.needs_by_actor.insert(
            actor_id(),
            [(
                crate::agent::NeedKind::Hunger,
                crate::agent::NeedState::initial(
                    crate::agent::NeedKind::Hunger,
                    200,
                    crate::agent::NeedChangeCause::FixtureInitial,
                ),
            )]
            .into_iter()
            .collect(),
        );
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_eat();

        let result = run_pipeline(
            &mut PipelineContext {
                registry: &registry,
                state: &mut state,
                agent_state: &mut agent_state,
                log: &mut log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
                ordering_key: ordering_key(),
            },
            &proposal(),
        );

        assert_eq!(result.report.status, ReportStatus::Accepted);
        assert_eq!(state.food_supplies[&food_supply_id()].servings, 1);
        assert_eq!(
            result.appended_events[0].event_type,
            EventKind::FoodConsumed
        );
    }
}
