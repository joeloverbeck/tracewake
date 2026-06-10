use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::agent::{NeedBand, NeedKind};
use crate::events::apply::ApplyError;
use crate::events::log::EventLog;
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ContentManifestId, EventId, WorkplaceId};
use crate::need_accounting::classify_actor_tick_regimes_with_start;
use crate::scheduler::OrderingKey;
use crate::state::{AgentState, PhysicalState, WorkplaceState};
use crate::time::SimTick;

pub fn build_work_start_events(
    state: &PhysicalState,
    agent_state: &AgentState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<Vec<EventEnvelope>, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    let actor = state.actors.get(&actor_id).ok_or_else(actor_missing)?;
    let workplace_id = workplace_target(proposal)?;
    let workplace = state.workplaces.get(&workplace_id).ok_or_else(|| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            "workplace_id",
            workplace_id.as_str(),
            "That workplace is not present.",
            "work target was missing from physical state",
        )
    })?;

    if actor.current_place_id != workplace.place_id {
        return Ok(vec![work_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            workplace,
            "access",
            "actor not at workplace",
        )]);
    }
    if !workplace.access_open {
        return Ok(vec![work_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            workplace,
            "access",
            "workplace access closed",
        )]);
    }
    if !workplace.assigned_actor_ids.is_empty() && !workplace.assigned_actor_ids.contains(&actor_id)
    {
        return Ok(vec![work_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            workplace,
            "physical",
            "actor not assigned to workplace",
        )]);
    }
    let Some(current_fatigue) = need_value(agent_state, &actor_id, NeedKind::Fatigue) else {
        return Ok(vec![work_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            workplace,
            "need",
            "fatigue state missing for work",
        )]);
    };
    if i32::from(current_fatigue) > workplace.max_fatigue_to_start {
        return Ok(vec![work_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            workplace,
            "need",
            "fatigue too high for work",
        )]);
    }
    let Some(current_hunger) = need_value(agent_state, &actor_id, NeedKind::Hunger) else {
        return Ok(vec![work_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            workplace,
            "need",
            "hunger state missing for work",
        )]);
    };
    if i32::from(current_hunger) > workplace.max_hunger_to_start {
        return Ok(vec![work_failed_event(
            proposal,
            ordering_key,
            content_manifest_id,
            workplace,
            "need",
            "hunger too high for work",
        )]);
    }

    Ok(vec![work_started_event(
        proposal,
        ordering_key,
        content_manifest_id,
        workplace,
    )])
}

pub fn build_work_completion_events(
    state: &PhysicalState,
    agent_state: &AgentState,
    log: &EventLog,
    work_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    completion_tick: SimTick,
) -> Result<Vec<EventEnvelope>, ApplyError> {
    let actor_id = work_started_event
        .actor_id
        .clone()
        .expect("work start event carries actor");
    let working_ticks = classify_actor_tick_regimes_with_start(
        log,
        &actor_id,
        work_started_event.sim_tick,
        completion_tick,
        Some(work_started_event),
    )
    .working_ticks;
    let fatigue_delta =
        payload_i32(work_started_event, "fatigue_delta_per_tick")? * working_ticks as i32;
    let hunger_delta =
        payload_i32(work_started_event, "hunger_delta_per_tick")? * working_ticks as i32;
    if let Some((blocker_kind, reason)) =
        work_completion_failure(state, agent_state, work_started_event)
    {
        let mut events = vec![work_failed_from_start_event(
            work_started_event,
            ordering_key,
            content_manifest_id,
            completion_tick,
            working_ticks,
            fatigue_delta,
            hunger_delta,
            blocker_kind,
            reason,
        )];
        events.push(need_delta_event(
            work_started_event,
            ordering_key,
            content_manifest_id,
            completion_tick,
            "fatigue",
            fatigue_delta,
        ));
        events.push(need_delta_event(
            work_started_event,
            ordering_key,
            content_manifest_id,
            completion_tick,
            "hunger",
            hunger_delta,
        ));
        return Ok(events);
    }
    Ok(vec![
        work_completed_event(
            work_started_event,
            ordering_key,
            content_manifest_id,
            completion_tick,
            working_ticks,
            fatigue_delta,
            hunger_delta,
        ),
        need_delta_event(
            work_started_event,
            ordering_key,
            content_manifest_id,
            completion_tick,
            "fatigue",
            fatigue_delta,
        ),
        need_delta_event(
            work_started_event,
            ordering_key,
            content_manifest_id,
            completion_tick,
            "hunger",
            hunger_delta,
        ),
    ])
}

fn work_completion_failure(
    state: &PhysicalState,
    agent_state: &AgentState,
    work_started_event: &EventEnvelope,
) -> Option<(&'static str, &'static str)> {
    let Some(actor_id) = work_started_event.actor_id.as_ref() else {
        return Some(("continuity", "actor_missing"));
    };
    let Some(actor) = state.actors.get(actor_id) else {
        return Some(("continuity", "actor_missing"));
    };
    let place_id = payload_value(work_started_event, "place_id");
    if actor.current_place_id.as_str() != place_id {
        return Some(("continuity", "actor_displaced"));
    }
    let workplace_id = payload_value(work_started_event, "workplace_id");
    let Some(workplace) = WorkplaceId::new(workplace_id)
        .ok()
        .and_then(|id| state.workplaces.get(&id))
    else {
        return Some(("continuity", "workplace_missing"));
    };
    if workplace.place_id.as_str() != place_id || !workplace.access_open {
        return Some(("continuity", "workplace_unusable"));
    }
    if has_severe_interrupting_need(agent_state, actor_id) {
        return Some(("need", "severe_need_pressure"));
    }
    None
}

fn has_severe_interrupting_need(agent_state: &AgentState, actor_id: &crate::ids::ActorId) -> bool {
    agent_state
        .needs_by_actor
        .get(actor_id)
        .is_some_and(|needs| {
            [NeedKind::Hunger, NeedKind::Safety].iter().any(|kind| {
                needs
                    .get(kind)
                    .is_some_and(|need| need.band() == NeedBand::Severe)
            })
        })
}

fn work_started_event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    workplace: &WorkplaceState,
) -> EventEnvelope {
    let actor_id = proposal.actor_id.as_ref().expect("actor checked");
    let expected_completion_tick = proposal
        .requested_tick
        .advance_by(workplace.work_duration_ticks);
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.work_block_started.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::WorkBlockStarted,
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
    event.participants = vec![actor_id.to_string(), workplace.workplace_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("workplace_id", workplace.workplace_id.as_str()),
        PayloadField::new("place_id", workplace.place_id.as_str()),
        PayloadField::new("duration_ticks", workplace.work_duration_ticks.to_string()),
        PayloadField::new(
            "expected_completion_tick",
            expected_completion_tick.value().to_string(),
        ),
        PayloadField::new("body_exclusive", "true"),
        PayloadField::new(
            "fatigue_delta_per_tick",
            workplace.fatigue_delta_per_tick.to_string(),
        ),
        PayloadField::new(
            "hunger_delta_per_tick",
            workplace.hunger_delta_per_tick.to_string(),
        ),
        PayloadField::new("output_tag", workplace.output_tag.as_str()),
        PayloadField::new("non_economic_output", "true"),
    ];
    event.effects_summary = "work block started; completion is duration scheduled".to_string();
    event
}

fn work_completed_event(
    work_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    completion_tick: SimTick,
    elapsed_ticks: u64,
    fatigue_delta: i32,
    hunger_delta: i32,
) -> EventEnvelope {
    let actor_id = work_started_event
        .actor_id
        .clone()
        .expect("work start event carries actor");
    let workplace_id = payload_value(work_started_event, "workplace_id");
    let output_tag = payload_value(work_started_event, "output_tag");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.work_block_completed.{}",
            work_started_event.event_id.as_str()
        ))
        .unwrap(),
        EventKind::WorkBlockCompleted,
        0,
        0,
        completion_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Event(work_started_event.event_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = work_started_event.proposal_id.clone();
    event.participants = vec![actor_id.to_string(), workplace_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("workplace_id", workplace_id),
        PayloadField::new("elapsed_ticks", elapsed_ticks.to_string()),
        PayloadField::new("fatigue_delta", fatigue_delta.to_string()),
        PayloadField::new("hunger_delta", hunger_delta.to_string()),
        PayloadField::new("output_tag", output_tag),
        PayloadField::new("non_economic_output", "true"),
    ];
    event.effects_summary = "work block completed with non-economic output marker".to_string();
    event
}

fn work_failed_event(
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    workplace: &WorkplaceState,
    blocker_kind: &str,
    reason: &str,
) -> EventEnvelope {
    let actor_id = proposal.actor_id.as_ref().expect("actor checked");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.work_block_failed.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::WorkBlockFailed,
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
    event.participants = vec![actor_id.to_string(), workplace.workplace_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("workplace_id", workplace.workplace_id.as_str()),
        PayloadField::new("blocker_kind", blocker_kind),
        PayloadField::new("reason", reason),
        PayloadField::new("completion_emitted", "false"),
    ];
    event.effects_summary = format!("work block failed: {reason}");
    event
}

#[allow(clippy::too_many_arguments)]
fn work_failed_from_start_event(
    work_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    failure_tick: SimTick,
    elapsed_ticks: u64,
    fatigue_delta: i32,
    hunger_delta: i32,
    blocker_kind: &str,
    reason: &str,
) -> EventEnvelope {
    let actor_id = work_started_event
        .actor_id
        .clone()
        .expect("work start event carries actor");
    let workplace_id = payload_value(work_started_event, "workplace_id");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.work_block_failed.{}",
            work_started_event.event_id.as_str()
        ))
        .unwrap(),
        EventKind::WorkBlockFailed,
        0,
        0,
        failure_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Event(work_started_event.event_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = work_started_event.proposal_id.clone();
    event.participants = vec![actor_id.to_string(), workplace_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("workplace_id", workplace_id),
        PayloadField::new("elapsed_ticks", elapsed_ticks.to_string()),
        PayloadField::new("fatigue_delta", fatigue_delta.to_string()),
        PayloadField::new("hunger_delta", hunger_delta.to_string()),
        PayloadField::new("blocker_kind", blocker_kind),
        PayloadField::new("reason", reason),
        PayloadField::new("completion_emitted", "false"),
    ];
    event.effects_summary = format!("work block failed during scheduled completion: {reason}");
    event
}

fn need_delta_event(
    work_started_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
    tick: SimTick,
    need_kind: &str,
    delta: i32,
) -> EventEnvelope {
    let actor_id = work_started_event
        .actor_id
        .clone()
        .expect("work start event carries actor");
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.work_need_delta.{}.{}",
            need_kind,
            work_started_event.event_id.as_str()
        ))
        .unwrap(),
        EventKind::NeedDeltaApplied,
        0,
        0,
        tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Event(work_started_event.event_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.participants = vec![actor_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("need_kind", need_kind),
        PayloadField::new("delta", delta.to_string()),
        PayloadField::new("cause_kind", "action_effect"),
        PayloadField::new("cause_action_id", "work_block"),
    ];
    event.effects_summary = format!("{need_kind} delta from elapsed work");
    event
}

fn workplace_target(proposal: &Proposal) -> Result<WorkplaceId, ActionRejection> {
    let target = proposal.target_ids.first().ok_or_else(|| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::TargetNotFound,
            "target_count",
            &proposal.target_ids.len().to_string(),
            "There is no workplace.",
            "work proposal did not include a workplace target",
        )
    })?;
    WorkplaceId::new(target).map_err(|_| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::UnsupportedTargetKind,
            "workplace_id",
            target,
            "That workplace is invalid.",
            "work target was not a stable workplace ID",
        )
    })
}

fn need_value(
    agent_state: &AgentState,
    actor_id: &crate::ids::ActorId,
    need_kind: NeedKind,
) -> Option<u16> {
    agent_state
        .needs_by_actor
        .get(actor_id)
        .and_then(|needs| needs.get(&need_kind))
        .map(crate::agent::NeedState::value)
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> &'a str {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
        .expect("work start event carries required payload")
}

fn payload_i32(event: &EventEnvelope, key: &'static str) -> Result<i32, ApplyError> {
    let value = event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
        .ok_or(ApplyError::MissingPayload(key))?;
    value.parse().map_err(|_| ApplyError::BadPayload {
        key,
        value: value.to_string(),
    })
}

fn actor_missing() -> ActionRejection {
    ActionRejection::new(
        PipelineStage::ActorLookup,
        ReasonCode::ActorNotFound,
        Vec::new(),
        "That actor cannot work.",
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
    use crate::actions::proposal::ProposalOrigin;
    use crate::actions::registry::ActionRegistry;
    use crate::actions::report::ReportStatus;
    use crate::agent::{NeedChangeCause, NeedState};
    use crate::events::log::EventLog;
    use crate::ids::{ActionId, ActorId, PlaceId, ProposalId, SleepAffordanceId};
    use crate::scheduler::{
        duration_completion_ordering_key, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use crate::state::{ActorBody, PlaceState, SleepAffordanceState};
    use std::collections::BTreeMap;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn workplace_id() -> WorkplaceId {
        WorkplaceId::new("workplace_office").unwrap()
    }

    fn place_id() -> PlaceId {
        PlaceId::new("office").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        state
            .places
            .insert(place_id(), PlaceState::new(place_id(), "Office"));
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), place_id()));
        state.workplaces.insert(
            workplace_id(),
            WorkplaceState::new(
                workplace_id(),
                place_id(),
                4,
                8,
                4,
                900,
                900,
                "service_completed_placeholder",
            ),
        );
        state.sleep_affordances.insert(
            SleepAffordanceId::new("bed_office").unwrap(),
            SleepAffordanceState::new(
                SleepAffordanceId::new("bed_office").unwrap(),
                place_id(),
                4,
                20,
                2,
            ),
        );
        state
    }

    fn displaced_state() -> PhysicalState {
        let mut state = state();
        let street = PlaceId::new("street").unwrap();
        state
            .places
            .insert(street.clone(), PlaceState::new(street.clone(), "Street"));
        state.actors.get_mut(&actor_id()).unwrap().current_place_id = street;
        state
    }

    fn agent_state_with_needs(fatigue: i32, hunger: i32) -> AgentState {
        let mut agent_state = AgentState::default();
        let mut needs = BTreeMap::new();
        needs.insert(
            NeedKind::Fatigue,
            NeedState::initial(NeedKind::Fatigue, fatigue, NeedChangeCause::FixtureInitial),
        );
        needs.insert(
            NeedKind::Hunger,
            NeedState::initial(NeedKind::Hunger, hunger, NeedChangeCause::FixtureInitial),
        );
        agent_state.needs_by_actor.insert(actor_id(), needs);
        agent_state
    }

    fn proposal() -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_work").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("work_block").unwrap(),
            SimTick::new(20),
        );
        proposal.target_ids.push(workplace_id().to_string());
        proposal
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(20),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new("work_block").unwrap(),
            vec![workplace_id().to_string()],
            "work_block",
        )
    }

    #[test]
    fn work_start_is_duration_based_and_non_economic() {
        let events = build_work_start_events(
            &state(),
            &agent_state_with_needs(100, 100),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(events[0].event_type, EventKind::WorkBlockStarted);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| { field.key == "expected_completion_tick" && field.value == "24" }));
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "non_economic_output" && field.value == "true"));
    }

    #[test]
    fn work_completion_emits_output_marker_and_need_costs() {
        let start = build_work_start_events(
            &state(),
            &agent_state_with_needs(100, 100),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap()
        .remove(0);
        let completion_key = duration_completion_ordering_key(
            &actor_id(),
            &ActionId::new("work_block").unwrap(),
            SimTick::new(24),
            0,
        );
        let events = build_work_completion_events(
            &state(),
            &agent_state_with_needs(100, 100),
            &EventLog::new(),
            &start,
            &completion_key,
            &ContentManifestId::new("phase3a_manifest").unwrap(),
            SimTick::new(24),
        )
        .unwrap();

        assert_eq!(events[0].event_type, EventKind::WorkBlockCompleted);
        assert!(events[0].payload.iter().any(
            |field| field.key == "output_tag" && field.value == "service_completed_placeholder"
        ));
        assert!(events.iter().any(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "need_kind" && field.value == "fatigue")
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "delta" && field.value == "32")
        }));
    }

    #[test]
    fn malformed_work_start_payload_returns_typed_error() {
        let mut start = build_work_start_events(
            &state(),
            &agent_state_with_needs(100, 100),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap()
        .remove(0);
        for field in &mut start.payload {
            if field.key == "fatigue_delta_per_tick" {
                field.value = "not_i32".to_string();
            }
        }
        let completion_key = duration_completion_ordering_key(
            &actor_id(),
            &ActionId::new("work_block").unwrap(),
            SimTick::new(24),
            0,
        );

        let error = build_work_completion_events(
            &state(),
            &agent_state_with_needs(100, 100),
            &EventLog::new(),
            &start,
            &completion_key,
            &ContentManifestId::new("phase3a_manifest").unwrap(),
            SimTick::new(24),
        )
        .unwrap_err();

        assert_eq!(
            error,
            ApplyError::BadPayload {
                key: "fatigue_delta_per_tick",
                value: "not_i32".to_string()
            }
        );
    }

    #[test]
    fn work_completion_fails_when_actor_displaced_with_prorated_costs() {
        let start = build_work_start_events(
            &state(),
            &agent_state_with_needs(100, 100),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap()
        .remove(0);
        let completion_key = duration_completion_ordering_key(
            &actor_id(),
            &ActionId::new("work_block").unwrap(),
            SimTick::new(22),
            0,
        );
        let events = build_work_completion_events(
            &displaced_state(),
            &agent_state_with_needs(100, 100),
            &EventLog::new(),
            &start,
            &completion_key,
            &ContentManifestId::new("phase3a_manifest").unwrap(),
            SimTick::new(22),
        )
        .unwrap();

        assert_eq!(events[0].event_type, EventKind::WorkBlockFailed);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "actor_displaced"));
        assert!(!events
            .iter()
            .any(|event| event.event_type == EventKind::WorkBlockCompleted));
        assert!(events.iter().any(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "need_kind" && field.value == "fatigue")
                && event
                    .payload
                    .iter()
                    .any(|field| field.key == "delta" && field.value == "16")
        }));
    }

    #[test]
    fn work_fails_from_authoritative_need_state_without_completion() {
        let events = build_work_start_events(
            &state(),
            &agent_state_with_needs(901, 100),
            &proposal(),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(events[0].event_type, EventKind::WorkBlockFailed);
        assert!(events[0]
            .payload
            .iter()
            .any(|field| field.key == "blocker_kind" && field.value == "need"));
        assert!(!events
            .iter()
            .any(|event| event.event_type == EventKind::WorkBlockCompleted));
    }

    #[test]
    fn work_forged_need_parameters_do_not_override_authoritative_state() {
        let mut state = state();
        let mut agent_state = agent_state_with_needs(901, 100);
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_work();
        let mut proposal = proposal();
        proposal
            .parameters
            .insert("current_fatigue".to_string(), "0".to_string());
        proposal
            .parameters
            .insert("current_hunger".to_string(), "0".to_string());

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
            &proposal,
        );

        assert_eq!(result.report.status, ReportStatus::Accepted);
        assert_eq!(
            result.appended_events[0].event_type,
            EventKind::WorkBlockFailed
        );
        assert!(result.appended_events[0]
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "fatigue too high for work"));
    }

    #[test]
    fn work_missing_or_malformed_need_parameters_do_not_default_to_safe_pass() {
        let mut state = state();
        let mut agent_state = agent_state_with_needs(100, 901);
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_work();
        let mut proposal = proposal();
        proposal
            .parameters
            .insert("current_fatigue".to_string(), "not-a-number".to_string());

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
            &proposal,
        );

        assert_eq!(result.report.status, ReportStatus::Accepted);
        assert_eq!(
            result.appended_events[0].event_type,
            EventKind::WorkBlockFailed
        );
        assert!(result.appended_events[0]
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "hunger too high for work"));
    }

    #[test]
    fn work_stale_proposal_fails_against_current_authoritative_state() {
        let mut state = state();
        let mut agent_state = agent_state_with_needs(100, 100);
        let mut proposal = proposal();
        proposal
            .parameters
            .insert("current_fatigue".to_string(), "100".to_string());
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_work();

        agent_state
            .needs_by_actor
            .get_mut(&actor_id())
            .unwrap()
            .insert(
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 901, NeedChangeCause::TickDelta),
            );

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
            &proposal,
        );

        assert_eq!(result.report.status, ReportStatus::Accepted);
        assert_eq!(
            result.appended_events[0].event_type,
            EventKind::WorkBlockFailed
        );
        assert!(result.appended_events[0]
            .payload
            .iter()
            .any(|field| field.key == "reason" && field.value == "fatigue too high for work"));
    }

    #[test]
    fn scheduler_origin_work_uses_shared_pipeline() {
        let mut state = state();
        let mut agent_state = agent_state_with_needs(100, 100);
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_work();
        let mut proposal = proposal();
        proposal.origin = ProposalOrigin::Scheduler;

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
            &proposal,
        );

        assert_eq!(result.report.status, ReportStatus::Accepted);
        assert_eq!(
            result.appended_events[0].event_type,
            EventKind::WorkBlockStarted
        );
    }

    #[test]
    fn overlapping_body_exclusive_action_is_reservation_conflict() {
        let mut state = state();
        let mut agent_state = agent_state_with_needs(100, 100);
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_work();
        registry.register_phase3a_sleep();

        let first = run_pipeline(
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
        assert_eq!(first.report.status, ReportStatus::Accepted);

        let mut sleep = Proposal::new(
            ProposalId::new("proposal_sleep_overlap").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id()),
            ActionId::new("sleep").unwrap(),
            SimTick::new(21),
        );
        sleep
            .parameters
            .insert("sleep_affordance_id".to_string(), "bed_office".to_string());
        let sleep_key = OrderingKey::new(
            SimTick::new(21),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(1),
            ActionId::new("sleep").unwrap(),
            Vec::new(),
            "sleep_overlap",
        );
        let second = run_pipeline(
            &mut PipelineContext {
                registry: &registry,
                state: &mut state,
                agent_state: &mut agent_state,
                log: &mut log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
                ordering_key: sleep_key,
            },
            &sleep,
        );

        assert_eq!(second.report.status, ReportStatus::Rejected);
        assert_eq!(
            second.report.reason_codes,
            vec![ReasonCode::ReservationConflict]
        );
        assert_eq!(
            second.report.failed_stage,
            Some(PipelineStage::ReservationConflictCheck)
        );
    }

    #[test]
    fn work_block_failed_closes_body_exclusive_reservation() {
        let mut state = state();
        let mut agent_state = agent_state_with_needs(100, 100);
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_work();
        registry.register_phase3a_sleep();

        let first = run_pipeline(
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
        assert_eq!(first.report.status, ReportStatus::Accepted);
        let work_started = first
            .appended_events
            .iter()
            .find(|event| event.event_type == EventKind::WorkBlockStarted)
            .expect("work starts")
            .clone();
        let completion_key = duration_completion_ordering_key(
            &actor_id(),
            &ActionId::new("work_block").unwrap(),
            SimTick::new(22),
            0,
        );
        let failure = build_work_completion_events(
            &displaced_state(),
            &agent_state,
            &log,
            &work_started,
            &completion_key,
            &ContentManifestId::new("phase3a_manifest").unwrap(),
            SimTick::new(22),
        )
        .unwrap()
        .into_iter()
        .find(|event| event.event_type == EventKind::WorkBlockFailed)
        .expect("displacement fails the work block");
        log.append(failure).unwrap();

        let mut sleep = Proposal::new(
            ProposalId::new("proposal_sleep_after_failed_work").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id()),
            ActionId::new("sleep").unwrap(),
            SimTick::new(23),
        );
        sleep
            .parameters
            .insert("sleep_place_id".to_string(), "office".to_string());
        sleep
            .parameters
            .insert("sleep_affordance_id".to_string(), "bed_office".to_string());
        let sleep_key = OrderingKey::new(
            SimTick::new(23),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(1),
            ActionId::new("sleep").unwrap(),
            Vec::new(),
            "sleep_after_failed_work",
        );
        let second = run_pipeline(
            &mut PipelineContext {
                registry: &registry,
                state: &mut state,
                agent_state: &mut agent_state,
                log: &mut log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
                ordering_key: sleep_key,
            },
            &sleep,
        );

        assert_eq!(second.report.status, ReportStatus::Accepted);
        assert!(second
            .appended_events
            .iter()
            .any(|event| event.event_type == EventKind::SleepStarted));
    }
}
