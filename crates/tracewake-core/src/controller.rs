use std::collections::BTreeMap;

use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, PayloadField};
use crate::ids::{ActorId, ContentManifestId, ControllerId, EventId};
use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use crate::state::{ControllerBinding, ControllerMode};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeControllerBinding {
    pub binding: ControllerBinding,
    pub created_at_tick: SimTick,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControllerBindings {
    bindings: BTreeMap<ControllerId, RuntimeControllerBinding>,
    next_sequence: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControllerError {
    ControllerUnbound(ControllerId),
    ControllerActorMismatch {
        controller_id: ControllerId,
        expected_actor_id: ActorId,
        actual_actor_id: Option<ActorId>,
    },
}

impl ControllerBindings {
    pub fn new() -> Self {
        Self {
            bindings: BTreeMap::new(),
            next_sequence: 0,
        }
    }

    pub fn attach(
        &mut self,
        controller_id: ControllerId,
        actor_id: ActorId,
        mode: ControllerMode,
        tick: SimTick,
        log: &mut EventLog,
        content_manifest_id: ContentManifestId,
    ) -> EventEnvelope {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        let binding = ControllerBinding {
            controller_id: controller_id.clone(),
            bound_actor_id: Some(actor_id.clone()),
            mode,
            binding_sequence: sequence,
        };
        self.bindings.insert(
            controller_id.clone(),
            RuntimeControllerBinding {
                binding,
                created_at_tick: tick,
            },
        );
        let event = controller_event(
            EventKind::ControllerAttached,
            controller_id,
            Some(actor_id),
            sequence,
            tick,
            content_manifest_id,
        );
        log.append(event).expect("controller event is versioned")
    }

    pub fn detach(
        &mut self,
        controller_id: &ControllerId,
        tick: SimTick,
        log: &mut EventLog,
        content_manifest_id: ContentManifestId,
    ) -> EventEnvelope {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.bindings.insert(
            controller_id.clone(),
            RuntimeControllerBinding {
                binding: ControllerBinding::detached(controller_id.clone(), sequence),
                created_at_tick: tick,
            },
        );
        let event = controller_event(
            EventKind::ControllerDetached,
            controller_id.clone(),
            None,
            sequence,
            tick,
            content_manifest_id,
        );
        log.append(event).expect("controller event is versioned")
    }

    pub fn binding(&self, controller_id: &ControllerId) -> Option<&RuntimeControllerBinding> {
        self.bindings.get(controller_id)
    }

    pub fn authorize(
        &self,
        controller_id: &ControllerId,
        actor_id: &ActorId,
    ) -> Result<(), ControllerError> {
        let binding = self
            .bindings
            .get(controller_id)
            .ok_or_else(|| ControllerError::ControllerUnbound(controller_id.clone()))?;
        if binding.binding.bound_actor_id.as_ref() == Some(actor_id) {
            Ok(())
        } else {
            Err(ControllerError::ControllerActorMismatch {
                controller_id: controller_id.clone(),
                expected_actor_id: actor_id.clone(),
                actual_actor_id: binding.binding.bound_actor_id.clone(),
            })
        }
    }
}

fn controller_event(
    kind: EventKind,
    controller_id: ControllerId,
    actor_id: Option<ActorId>,
    sequence: u64,
    tick: SimTick,
    content_manifest_id: ContentManifestId,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_v1(
        EventId::new(format!(
            "event.{}.{}.{}",
            kind.stable_id(),
            controller_id.as_str(),
            sequence
        ))
        .unwrap(),
        kind,
        0,
        0,
        tick,
        OrderingKey::new(
            tick,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Controller(controller_id.clone()),
            ProposalSequence::new(sequence),
            crate::ids::ActionId::new(kind.stable_id()).unwrap(),
            actor_id
                .as_ref()
                .map(|id| vec![id.to_string()])
                .unwrap_or_default(),
            "controller_binding",
        ),
        content_manifest_id,
    );
    event.payload = vec![
        PayloadField::new("controller_id", controller_id.as_str()),
        PayloadField::new("binding_sequence", sequence.to_string()),
    ];
    if let Some(actor_id) = actor_id {
        event.actor_id = Some(actor_id.clone());
        event
            .payload
            .push(PayloadField::new("actor_id", actor_id.as_str()));
        event.participants.push(actor_id.to_string());
    }
    event.effects_summary = "controller binding metadata changed".to_string();
    event
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::checksum::{compute_physical_checksum, ChecksumContext};
    use crate::events::apply::apply_event;
    use crate::ids::{ActionId, ContentVersion, FixtureId, PlaceId, ProposalId};
    use crate::projections::build_embodied_view_model;
    use crate::state::{ActorBody, PhysicalState, PlaceState};

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn controller_id() -> ControllerId {
        ControllerId::new("controller_human").unwrap()
    }

    fn content_manifest_id() -> ContentManifestId {
        ContentManifestId::new("phase1_manifest").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
        let place_id = PlaceId::new("shop_front").unwrap();
        state.places.insert(
            place_id.clone(),
            PlaceState::new(place_id.clone(), "Shop front"),
        );
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(actor_id("actor_tomas"), place_id),
        );
        state
    }

    fn checksum_context() -> ChecksumContext {
        ChecksumContext {
            fixture_id: FixtureId::new("debug_attach_001").unwrap(),
            content_version: ContentVersion::new("phase1_manifest").unwrap(),
            sim_tick: SimTick::ZERO,
            world_stream_position_applied: 0,
        }
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id("actor_tomas")),
            ProposalSequence::new(0),
            ActionId::new("look").unwrap(),
            Vec::new(),
            "tie",
        )
    }

    #[test]
    fn binding_lifecycle_changes_no_actor_component_or_physical_checksum() {
        let state = state();
        let before_actor = state.actors[&actor_id("actor_tomas")].clone();
        let before_checksum = compute_physical_checksum(&state, &checksum_context()).checksum;
        let mut bindings = ControllerBindings::new();
        let mut log = EventLog::new();

        let attached = bindings.attach(
            controller_id(),
            actor_id("actor_tomas"),
            ControllerMode::Embodied,
            SimTick::ZERO,
            &mut log,
            content_manifest_id(),
        );
        bindings.attach(
            controller_id(),
            actor_id("actor_tomas"),
            ControllerMode::Debug,
            SimTick::new(1),
            &mut log,
            content_manifest_id(),
        );
        bindings.detach(
            &controller_id(),
            SimTick::new(2),
            &mut log,
            content_manifest_id(),
        );
        let after_checksum = compute_physical_checksum(&state, &checksum_context()).checksum;

        assert_eq!(state.actors[&actor_id("actor_tomas")], before_actor);
        assert_eq!(before_checksum, after_checksum);
        assert_eq!(attached.stream, crate::events::EventStream::Controller);
        assert!(log
            .events()
            .iter()
            .all(|event| event.stream == crate::events::EventStream::Controller));
    }

    #[test]
    fn controller_event_application_is_physical_noop() {
        let mut state = state();
        let before = state.clone();
        let mut bindings = ControllerBindings::new();
        let mut log = EventLog::new();
        let event = bindings.attach(
            controller_id(),
            actor_id("actor_tomas"),
            ControllerMode::Embodied,
            SimTick::ZERO,
            &mut log,
            content_manifest_id(),
        );

        assert!(matches!(
            apply_event(&mut state, &event),
            Ok(crate::events::apply::ApplyOutcome::NonWorldNoOp)
        ));
        assert_eq!(state, before);
    }

    #[test]
    fn human_after_authorization_matches_scheduler_validation() {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_inspect_wait();
        let mut bindings = ControllerBindings::new();
        let mut binding_log = EventLog::new();
        bindings.attach(
            controller_id(),
            actor_id("actor_tomas"),
            ControllerMode::Embodied,
            SimTick::ZERO,
            &mut binding_log,
            content_manifest_id(),
        );

        let mut human_state = state();
        let mut human_log = EventLog::new();
        let mut human_proposal = Proposal::new(
            ProposalId::new("proposal_look").unwrap(),
            ProposalOrigin::Human,
            Some(actor_id("actor_tomas")),
            ActionId::new("look").unwrap(),
            SimTick::ZERO,
        );
        human_proposal
            .parameters
            .insert("controller_id".to_string(), controller_id().to_string());
        let mut human_context = PipelineContext {
            registry: &registry,
            state: &mut human_state,
            log: &mut human_log,
            controller_bindings: Some(&bindings),
            content_manifest_id: content_manifest_id(),
            ordering_key: ordering_key(),
        };
        let human = run_pipeline(&mut human_context, &human_proposal);

        let mut scheduler_state = state();
        let mut scheduler_log = EventLog::new();
        let scheduler_proposal = Proposal::new(
            ProposalId::new("proposal_look").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id("actor_tomas")),
            ActionId::new("look").unwrap(),
            SimTick::ZERO,
        );
        let mut scheduler_context = PipelineContext {
            registry: &registry,
            state: &mut scheduler_state,
            log: &mut scheduler_log,
            controller_bindings: None,
            content_manifest_id: content_manifest_id(),
            ordering_key: ordering_key(),
        };
        let scheduler = run_pipeline(&mut scheduler_context, &scheduler_proposal);

        assert_eq!(human.report.status, scheduler.report.status);
        assert_eq!(human.report.reason_codes, scheduler.report.reason_codes);
        assert_eq!(human.appended_events, scheduler.appended_events);
    }

    #[test]
    fn human_binding_adds_no_extra_semantic_actions() {
        let state = state();
        let before =
            build_embodied_view_model(&state, &actor_id("actor_tomas"), SimTick::ZERO, None)
                .unwrap();
        let mut bindings = ControllerBindings::new();
        let mut log = EventLog::new();
        bindings.attach(
            controller_id(),
            actor_id("actor_tomas"),
            ControllerMode::Embodied,
            SimTick::ZERO,
            &mut log,
            content_manifest_id(),
        );
        let after =
            build_embodied_view_model(&state, &actor_id("actor_tomas"), SimTick::ZERO, None)
                .unwrap();

        assert_eq!(before.semantic_actions, after.semantic_actions);
    }
}
