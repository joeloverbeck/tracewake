pub mod apply;
mod envelope;
pub mod log;

pub use envelope::{
    BeliefUpdatedPayload, ContainerCheckedPayload, EventCause, EventEnvelope,
    EventEnvelopeParseError, EventKind, EventKindMetadata, EventStream,
    ExpectationContradictedPayload, InitialBeliefSeededPayload, InitialBeliefSourceKind,
    ObservationRecordedPayload, PayloadField, RandomDrawRef,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{ActionId, ActorId, ContentManifestId, EventId, ProcessId};
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::time::SimTick;

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(3),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(ActorId::new("actor_tomas").unwrap()),
            ProposalSequence::new(7),
            ActionId::new("open").unwrap(),
            vec!["strongbox_tomas".to_string()],
            "tie_a",
        )
    }

    #[test]
    fn event_kinds_map_to_one_stream_and_non_world_is_non_mutating() {
        for kind in EventKind::all() {
            let metadata = kind.metadata();
            assert_eq!(metadata.kind, *kind);
            if metadata.stream != EventStream::World {
                assert!(!metadata.physical_mutating);
            }
        }
    }

    #[test]
    fn registry_covers_every_known_kind() {
        let registry = EventKind::registry();
        let from_all: Vec<_> = EventKind::all()
            .iter()
            .map(|kind| kind.metadata())
            .collect();

        assert_eq!(registry, from_all);
        assert!(registry
            .iter()
            .any(|entry| entry.kind == EventKind::ActorMoved));
        assert!(registry
            .iter()
            .any(|entry| entry.kind == EventKind::ReplayProjectionRebuilt));
    }

    #[test]
    fn envelope_round_trips_with_stable_schema_version() {
        let mut envelope = EventEnvelope::new_v1(
            EventId::new("event_0001").unwrap(),
            EventKind::ActionRejected,
            0,
            2,
            SimTick::new(3),
            ordering_key(),
            ContentManifestId::new("phase1_manifest").unwrap(),
        );
        envelope.process_id = Some(ProcessId::new("validation_pipeline").unwrap());
        envelope.participants = vec!["actor_tomas".to_string(), "strongbox_tomas".to_string()];
        envelope.causes = vec![EventCause::Process(ProcessId::new("fixture_root").unwrap())];
        envelope.payload = vec![PayloadField::new("reason_code", "container_closed")];
        envelope.effects_summary = "rejected before mutation".to_string();

        let bytes = envelope.serialize_canonical();
        let round_tripped = EventEnvelope::deserialize_canonical(&bytes).unwrap();

        assert_eq!(round_tripped, envelope);
        assert_eq!(
            round_tripped.event_schema_version.as_str(),
            "event_schema_v1"
        );
    }

    #[test]
    fn unknown_schema_version_is_flagged() {
        let mut envelope = EventEnvelope::new_v1(
            EventId::new("event_0001").unwrap(),
            EventKind::ActorMoved,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            ContentManifestId::new("phase1_manifest").unwrap(),
        );

        assert!(envelope.has_supported_schema_version());
        envelope.event_schema_version =
            crate::ids::SchemaVersion::new("event_schema_v999").unwrap();
        assert!(!envelope.has_supported_schema_version());
    }
}
