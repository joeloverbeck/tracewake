use crate::events::{EventEnvelope, EventEnvelopeParseError};
use crate::ids::EventId;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventLog {
    events: Vec<EventEnvelope>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventLogError {
    UnsupportedSchemaVersion(String),
    Parse(EventEnvelopeParseError),
    BadHex,
    InvalidUtf8,
    GlobalOrderMismatch { expected: u64, actual: u64 },
    StreamPositionMismatch { expected: u64, actual: u64 },
    DuplicateEventId(EventId),
}

impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn append(&mut self, mut event: EventEnvelope) -> Result<EventEnvelope, EventLogError> {
        if !event.has_supported_schema_version() {
            return Err(EventLogError::UnsupportedSchemaVersion(
                event.event_schema_version.as_str().to_string(),
            ));
        }
        if self.contains_event_id(&event.event_id) {
            return Err(EventLogError::DuplicateEventId(event.event_id));
        }

        event.global_order = self.events.len() as u64;
        event.stream_position = self
            .events
            .iter()
            .filter(|existing| existing.stream == event.stream)
            .count() as u64;
        self.events.push(event.clone());
        Ok(event)
    }

    fn append_deserialized(
        &mut self,
        event: EventEnvelope,
    ) -> Result<EventEnvelope, EventLogError> {
        if !event.has_supported_schema_version() {
            return Err(EventLogError::UnsupportedSchemaVersion(
                event.event_schema_version.as_str().to_string(),
            ));
        }
        if self.contains_event_id(&event.event_id) {
            return Err(EventLogError::DuplicateEventId(event.event_id));
        }
        let expected_global_order = self.events.len() as u64;
        if event.global_order != expected_global_order {
            return Err(EventLogError::GlobalOrderMismatch {
                expected: expected_global_order,
                actual: event.global_order,
            });
        }
        let expected_stream_position = self
            .events
            .iter()
            .filter(|existing| existing.stream == event.stream)
            .count() as u64;
        if event.stream_position != expected_stream_position {
            return Err(EventLogError::StreamPositionMismatch {
                expected: expected_stream_position,
                actual: event.stream_position,
            });
        }
        self.events.push(event.clone());
        Ok(event)
    }

    pub fn events(&self) -> &[EventEnvelope] {
        &self.events
    }

    pub(crate) fn contains_event_id(&self, event_id: &EventId) -> bool {
        self.events
            .iter()
            .any(|existing| &existing.event_id == event_id)
    }

    #[cfg(test)]
    pub(crate) fn from_ordered_events_for_replay_tests(events: Vec<EventEnvelope>) -> Self {
        Self { events }
    }

    pub fn serialize_canonical(&self) -> Vec<u8> {
        self.events
            .iter()
            .map(|event| encode_hex(&event.serialize_canonical()))
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes()
    }

    pub fn deserialize_canonical(bytes: &[u8]) -> Result<Self, EventLogError> {
        let text = std::str::from_utf8(bytes).map_err(|_| EventLogError::InvalidUtf8)?;
        let mut log = EventLog::new();
        if text.is_empty() {
            return Ok(log);
        }

        for line in text.lines() {
            let event_bytes = decode_hex(line)?;
            let event = EventEnvelope::deserialize_canonical(&event_bytes)
                .map_err(envelope_parse_error_for_log)?;
            log.append_deserialized(event)?;
        }
        Ok(log)
    }
}

fn envelope_parse_error_for_log(error: EventEnvelopeParseError) -> EventLogError {
    match error {
        EventEnvelopeParseError::UnsupportedSchemaVersion(version) => {
            EventLogError::UnsupportedSchemaVersion(version)
        }
        other => EventLogError::Parse(other),
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn decode_hex(value: &str) -> Result<Vec<u8>, EventLogError> {
    if !value.len().is_multiple_of(2) {
        return Err(EventLogError::BadHex);
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|chunk| {
            let hex = std::str::from_utf8(chunk).map_err(|_| EventLogError::BadHex)?;
            u8::from_str_radix(hex, 16).map_err(|_| EventLogError::BadHex)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventKind;
    use crate::ids::{ActionId, ActorId, ContentManifestId, EventId};
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::time::SimTick;

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(ActorId::new("actor_tomas").unwrap()),
            ProposalSequence::new(0),
            ActionId::new("wait").unwrap(),
            vec!["1_tick".to_string()],
            "tie",
        )
    }

    fn event(id: &str, kind: EventKind) -> EventEnvelope {
        EventEnvelope::new_v1(
            EventId::new(id).unwrap(),
            kind,
            99,
            99,
            SimTick::ZERO,
            ordering_key(),
            ContentManifestId::new("phase1_manifest").unwrap(),
        )
    }

    #[test]
    fn append_assigns_monotonic_positions_and_preserves_order() {
        let mut log = EventLog::new();

        log.append(event("event_0001", EventKind::ActorWaited))
            .unwrap();
        log.append(event("event_0002", EventKind::ActionRejected))
            .unwrap();
        log.append(event("event_0003", EventKind::TimeAdvanced))
            .unwrap();

        assert_eq!(log.events()[0].global_order, 0);
        assert_eq!(log.events()[0].stream_position, 0);
        assert_eq!(log.events()[1].global_order, 1);
        assert_eq!(log.events()[1].stream_position, 0);
        assert_eq!(log.events()[2].global_order, 2);
        assert_eq!(log.events()[2].stream_position, 1);
        assert_eq!(log.events()[0].event_id.as_str(), "event_0001");
        assert_eq!(log.events()[2].event_id.as_str(), "event_0003");
    }

    #[test]
    fn log_serialization_round_trips() {
        let mut log = EventLog::new();
        log.append(event("event_0001", EventKind::ActorWaited))
            .unwrap();
        log.append(event("event_0002", EventKind::ActionRejected))
            .unwrap();

        let bytes = log.serialize_canonical();
        let round_tripped = EventLog::deserialize_canonical(&bytes).unwrap();

        assert_eq!(round_tripped, log);
    }

    #[test]
    fn append_rejects_duplicate_event_id() {
        let mut log = EventLog::new();
        log.append(event("event_duplicate", EventKind::ActorWaited))
            .unwrap();

        let error = log
            .append(event("event_duplicate", EventKind::ActionRejected))
            .unwrap_err();

        assert_eq!(
            error,
            EventLogError::DuplicateEventId(EventId::new("event_duplicate").unwrap())
        );
        assert_eq!(log.events().len(), 1);
    }

    #[test]
    fn deserialization_rejects_duplicate_event_id() {
        let mut log = EventLog::new();
        log.append(event("event_duplicate", EventKind::ActorWaited))
            .unwrap();
        log.append(event("event_other", EventKind::ActionRejected))
            .unwrap();
        let mut duplicate = log.events()[1].clone();
        duplicate.event_id = EventId::new("event_duplicate").unwrap();
        let bytes = [log.events()[0].clone(), duplicate]
            .into_iter()
            .map(|event| encode_hex(&event.serialize_canonical()))
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes();

        let error = EventLog::deserialize_canonical(&bytes).unwrap_err();

        assert_eq!(
            error,
            EventLogError::DuplicateEventId(EventId::new("event_duplicate").unwrap())
        );
    }

    #[test]
    fn deserialization_rejects_reordered_global_order() {
        let mut log = EventLog::new();
        log.append(event("event_0001", EventKind::ActorWaited))
            .unwrap();
        log.append(event("event_0002", EventKind::ActionRejected))
            .unwrap();
        let text = String::from_utf8(log.serialize_canonical()).unwrap();
        let mut lines = text.lines().collect::<Vec<_>>();
        lines.swap(0, 1);

        let error = EventLog::deserialize_canonical(lines.join("\n").as_bytes()).unwrap_err();

        assert_eq!(
            error,
            EventLogError::GlobalOrderMismatch {
                expected: 0,
                actual: 1
            }
        );
    }
}
