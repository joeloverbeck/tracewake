use crate::events::{EventEnvelope, EventEnvelopeParseError};

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

        event.global_order = self.events.len() as u64;
        event.stream_position = self
            .events
            .iter()
            .filter(|existing| existing.stream == event.stream)
            .count() as u64;
        self.events.push(event.clone());
        Ok(event)
    }

    pub fn events(&self) -> &[EventEnvelope] {
        &self.events
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
            let event =
                EventEnvelope::deserialize_canonical(&event_bytes).map_err(EventLogError::Parse)?;
            log.append(event)?;
        }
        Ok(log)
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
}
