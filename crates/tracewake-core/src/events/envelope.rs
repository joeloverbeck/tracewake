use crate::ids::{
    ActionId, ActorId, ContentManifestId, ControllerId, EventId, PlaceId, ProcessId, ProposalId,
    SchemaVersion, ValidationReportId,
};
use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use crate::time::SimTick;

pub const EVENT_SCHEMA_V1: &str = "event_schema_v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventStream {
    World,
    Diagnostic,
    Controller,
    ReplayDebug,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventKind {
    ControllerAttached,
    ControllerDetached,
    ActorMoved,
    DoorOpened,
    DoorClosed,
    ContainerOpened,
    ContainerClosed,
    ItemRemovedFromContainer,
    ItemTakenFromPlace,
    ItemPlacedInContainer,
    ItemPlacedInPlace,
    ActorWaited,
    TimeAdvanced,
    ActionStarted,
    ActionFailed,
    ActionRejected,
    NoHumanAdvanceStarted,
    NoHumanAdvanceCompleted,
    ReplayProjectionRebuilt,
}

impl EventKind {
    pub const fn all() -> &'static [EventKind] {
        &[
            EventKind::ControllerAttached,
            EventKind::ControllerDetached,
            EventKind::ActorMoved,
            EventKind::DoorOpened,
            EventKind::DoorClosed,
            EventKind::ContainerOpened,
            EventKind::ContainerClosed,
            EventKind::ItemRemovedFromContainer,
            EventKind::ItemTakenFromPlace,
            EventKind::ItemPlacedInContainer,
            EventKind::ItemPlacedInPlace,
            EventKind::ActorWaited,
            EventKind::TimeAdvanced,
            EventKind::ActionStarted,
            EventKind::ActionFailed,
            EventKind::ActionRejected,
            EventKind::NoHumanAdvanceStarted,
            EventKind::NoHumanAdvanceCompleted,
            EventKind::ReplayProjectionRebuilt,
        ]
    }

    pub fn registry() -> Vec<EventKindMetadata> {
        Self::all().iter().map(|kind| kind.metadata()).collect()
    }

    pub const fn metadata(self) -> EventKindMetadata {
        EventKindMetadata {
            kind: self,
            stream: self.stream(),
            physical_mutating: self.physical_mutating(),
        }
    }

    pub const fn stream(self) -> EventStream {
        match self {
            EventKind::ControllerAttached | EventKind::ControllerDetached => {
                EventStream::Controller
            }
            EventKind::ActionStarted
            | EventKind::ActionFailed
            | EventKind::ActionRejected
            | EventKind::NoHumanAdvanceStarted
            | EventKind::NoHumanAdvanceCompleted => EventStream::Diagnostic,
            EventKind::ReplayProjectionRebuilt => EventStream::ReplayDebug,
            EventKind::ActorMoved
            | EventKind::DoorOpened
            | EventKind::DoorClosed
            | EventKind::ContainerOpened
            | EventKind::ContainerClosed
            | EventKind::ItemRemovedFromContainer
            | EventKind::ItemTakenFromPlace
            | EventKind::ItemPlacedInContainer
            | EventKind::ItemPlacedInPlace
            | EventKind::ActorWaited
            | EventKind::TimeAdvanced => EventStream::World,
        }
    }

    pub const fn physical_mutating(self) -> bool {
        matches!(
            self,
            EventKind::ActorMoved
                | EventKind::DoorOpened
                | EventKind::DoorClosed
                | EventKind::ContainerOpened
                | EventKind::ContainerClosed
                | EventKind::ItemRemovedFromContainer
                | EventKind::ItemTakenFromPlace
                | EventKind::ItemPlacedInContainer
                | EventKind::ItemPlacedInPlace
                | EventKind::ActorWaited
                | EventKind::TimeAdvanced
        )
    }

    pub const fn stable_id(self) -> &'static str {
        match self {
            EventKind::ControllerAttached => "controller_attached",
            EventKind::ControllerDetached => "controller_detached",
            EventKind::ActorMoved => "actor_moved",
            EventKind::DoorOpened => "door_opened",
            EventKind::DoorClosed => "door_closed",
            EventKind::ContainerOpened => "container_opened",
            EventKind::ContainerClosed => "container_closed",
            EventKind::ItemRemovedFromContainer => "item_removed_from_container",
            EventKind::ItemTakenFromPlace => "item_taken_from_place",
            EventKind::ItemPlacedInContainer => "item_placed_in_container",
            EventKind::ItemPlacedInPlace => "item_placed_in_place",
            EventKind::ActorWaited => "actor_waited",
            EventKind::TimeAdvanced => "time_advanced",
            EventKind::ActionStarted => "action_started",
            EventKind::ActionFailed => "action_failed",
            EventKind::ActionRejected => "action_rejected",
            EventKind::NoHumanAdvanceStarted => "no_human_advance_started",
            EventKind::NoHumanAdvanceCompleted => "no_human_advance_completed",
            EventKind::ReplayProjectionRebuilt => "replay_projection_rebuilt",
        }
    }

    fn from_stable_id(value: &str) -> Option<Self> {
        Self::all()
            .iter()
            .copied()
            .find(|kind| kind.stable_id() == value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventKindMetadata {
    pub kind: EventKind,
    pub stream: EventStream,
    pub physical_mutating: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventCause {
    Event(EventId),
    Proposal(ProposalId),
    ValidationReport(ValidationReportId),
    Process(ProcessId),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RandomDrawRef {
    pub scope: String,
    pub draw_id: String,
    pub value: String,
}

impl RandomDrawRef {
    pub fn new(
        scope: impl Into<String>,
        draw_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            scope: scope.into(),
            draw_id: draw_id.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PayloadField {
    pub key: String,
    pub value: String,
}

impl PayloadField {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventEnvelope {
    pub event_id: EventId,
    pub event_type: EventKind,
    pub event_schema_version: SchemaVersion,
    pub stream: EventStream,
    pub stream_position: u64,
    pub global_order: u64,
    pub sim_tick: SimTick,
    pub ordering_key: OrderingKey,
    pub actor_id: Option<ActorId>,
    pub process_id: Option<ProcessId>,
    pub participants: Vec<String>,
    pub place_id: Option<PlaceId>,
    pub causes: Vec<EventCause>,
    pub proposal_id: Option<ProposalId>,
    pub validation_report_id: Option<ValidationReportId>,
    pub random_draws: Vec<RandomDrawRef>,
    pub payload: Vec<PayloadField>,
    pub effects_summary: String,
    pub content_manifest_id: ContentManifestId,
    pub checksum_after: Option<String>,
}

impl EventEnvelope {
    pub fn new_v1(
        event_id: EventId,
        event_type: EventKind,
        stream_position: u64,
        global_order: u64,
        sim_tick: SimTick,
        ordering_key: OrderingKey,
        content_manifest_id: ContentManifestId,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            stream: event_type.stream(),
            stream_position,
            global_order,
            sim_tick,
            ordering_key,
            actor_id: None,
            process_id: None,
            participants: Vec::new(),
            place_id: None,
            causes: Vec::new(),
            proposal_id: None,
            validation_report_id: None,
            random_draws: Vec::new(),
            payload: Vec::new(),
            effects_summary: String::new(),
            content_manifest_id,
            checksum_after: None,
        }
    }

    pub fn has_supported_schema_version(&self) -> bool {
        self.event_schema_version.as_str() == EVENT_SCHEMA_V1
    }

    pub fn serialize_canonical(&self) -> Vec<u8> {
        let fields = [
            ("event_id", encode(self.event_id.as_str())),
            ("event_type", encode(self.event_type.stable_id())),
            (
                "event_schema_version",
                encode(self.event_schema_version.as_str()),
            ),
            ("stream", encode(stream_id(self.stream))),
            ("stream_position", self.stream_position.to_string()),
            ("global_order", self.global_order.to_string()),
            ("sim_tick", self.sim_tick.value().to_string()),
            (
                "ordering_key",
                encode(&serialize_ordering_key(&self.ordering_key)),
            ),
            (
                "actor_id",
                encode_opt(self.actor_id.as_ref().map(ActorId::as_str)),
            ),
            (
                "process_id",
                encode_opt(self.process_id.as_ref().map(ProcessId::as_str)),
            ),
            ("participants", encode_vec(&self.participants)),
            (
                "place_id",
                encode_opt(self.place_id.as_ref().map(PlaceId::as_str)),
            ),
            (
                "causes",
                encode_vec(&self.causes.iter().map(serialize_cause).collect::<Vec<_>>()),
            ),
            (
                "proposal_id",
                encode_opt(self.proposal_id.as_ref().map(ProposalId::as_str)),
            ),
            (
                "validation_report_id",
                encode_opt(
                    self.validation_report_id
                        .as_ref()
                        .map(ValidationReportId::as_str),
                ),
            ),
            (
                "random_draws",
                encode_vec(
                    &self
                        .random_draws
                        .iter()
                        .map(serialize_random_draw)
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "payload",
                encode_vec(
                    &self
                        .payload
                        .iter()
                        .map(serialize_payload)
                        .collect::<Vec<_>>(),
                ),
            ),
            ("effects_summary", encode(&self.effects_summary)),
            (
                "content_manifest_id",
                encode(self.content_manifest_id.as_str()),
            ),
            ("checksum_after", encode_opt(self.checksum_after.as_deref())),
        ];

        fields
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes()
    }

    pub fn deserialize_canonical(bytes: &[u8]) -> Result<Self, EventEnvelopeParseError> {
        let text = std::str::from_utf8(bytes).map_err(|_| EventEnvelopeParseError::InvalidUtf8)?;
        let mut map = std::collections::BTreeMap::new();
        for line in text.lines() {
            let (key, value) = line
                .split_once('=')
                .ok_or(EventEnvelopeParseError::MalformedField)?;
            map.insert(key, value);
        }

        let event_type = EventKind::from_stable_id(&decode(required(&map, "event_type")?)?)
            .ok_or(EventEnvelopeParseError::UnknownEventKind)?;
        let ordering_key = deserialize_ordering_key(&decode(required(&map, "ordering_key")?)?)?;

        Ok(Self {
            event_id: EventId::new(decode(required(&map, "event_id")?)?)?,
            event_type,
            event_schema_version: SchemaVersion::new(decode(required(
                &map,
                "event_schema_version",
            )?)?)?,
            stream: stream_from_id(&decode(required(&map, "stream")?)?)
                .ok_or(EventEnvelopeParseError::UnknownStream)?,
            stream_position: required(&map, "stream_position")?.parse()?,
            global_order: required(&map, "global_order")?.parse()?,
            sim_tick: SimTick::new(required(&map, "sim_tick")?.parse()?),
            ordering_key,
            actor_id: decode_opt(required(&map, "actor_id")?)?
                .map(ActorId::new)
                .transpose()?,
            process_id: decode_opt(required(&map, "process_id")?)?
                .map(ProcessId::new)
                .transpose()?,
            participants: decode_vec(required(&map, "participants")?)?,
            place_id: decode_opt(required(&map, "place_id")?)?
                .map(PlaceId::new)
                .transpose()?,
            causes: decode_vec(required(&map, "causes")?)?
                .into_iter()
                .map(|value| deserialize_cause(&value))
                .collect::<Result<_, _>>()?,
            proposal_id: decode_opt(required(&map, "proposal_id")?)?
                .map(ProposalId::new)
                .transpose()?,
            validation_report_id: decode_opt(required(&map, "validation_report_id")?)?
                .map(ValidationReportId::new)
                .transpose()?,
            random_draws: decode_vec(required(&map, "random_draws")?)?
                .into_iter()
                .map(|value| deserialize_random_draw(&value))
                .collect::<Result<_, _>>()?,
            payload: decode_vec(required(&map, "payload")?)?
                .into_iter()
                .map(|value| deserialize_payload(&value))
                .collect::<Result<_, _>>()?,
            effects_summary: decode(required(&map, "effects_summary")?)?,
            content_manifest_id: ContentManifestId::new(decode(required(
                &map,
                "content_manifest_id",
            )?)?)?,
            checksum_after: decode_opt(required(&map, "checksum_after")?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventEnvelopeParseError {
    InvalidUtf8,
    MalformedField,
    MissingField(&'static str),
    BadHex,
    UnknownEventKind,
    UnknownStream,
    UnknownSchedulePhase,
    UnknownSchedulerSource,
    UnknownCause,
    InvalidTuple,
    Id(crate::ids::IdError),
    Number,
}

impl From<crate::ids::IdError> for EventEnvelopeParseError {
    fn from(value: crate::ids::IdError) -> Self {
        Self::Id(value)
    }
}

impl From<std::num::ParseIntError> for EventEnvelopeParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::Number
    }
}

fn required<'a>(
    map: &'a std::collections::BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<&'a str, EventEnvelopeParseError> {
    map.get(key)
        .copied()
        .ok_or(EventEnvelopeParseError::MissingField(key))
}

fn stream_id(stream: EventStream) -> &'static str {
    match stream {
        EventStream::World => "world",
        EventStream::Diagnostic => "diagnostic",
        EventStream::Controller => "controller",
        EventStream::ReplayDebug => "replay_debug",
    }
}

fn stream_from_id(value: &str) -> Option<EventStream> {
    match value {
        "world" => Some(EventStream::World),
        "diagnostic" => Some(EventStream::Diagnostic),
        "controller" => Some(EventStream::Controller),
        "replay_debug" => Some(EventStream::ReplayDebug),
        _ => None,
    }
}

fn serialize_ordering_key(key: &OrderingKey) -> String {
    [
        key.sim_tick.value().to_string(),
        schedule_phase_id(&key.phase).to_string(),
        serialize_source_id(&key.source_id),
        key.proposal_sequence.value().to_string(),
        key.action_id.as_str().to_string(),
        key.target_ids.join(","),
        key.final_tie_breaker.clone(),
    ]
    .into_iter()
    .map(|part| encode(&part))
    .collect::<Vec<_>>()
    .join("|")
}

fn deserialize_ordering_key(value: &str) -> Result<OrderingKey, EventEnvelopeParseError> {
    let parts = value
        .split('|')
        .map(decode)
        .collect::<Result<Vec<_>, _>>()?;
    if parts.len() != 7 {
        return Err(EventEnvelopeParseError::InvalidTuple);
    }

    Ok(OrderingKey::new(
        SimTick::new(parts[0].parse()?),
        schedule_phase_from_id(&parts[1]).ok_or(EventEnvelopeParseError::UnknownSchedulePhase)?,
        deserialize_source_id(&parts[2])?,
        ProposalSequence::new(parts[3].parse()?),
        ActionId::new(parts[4].clone())?,
        if parts[5].is_empty() {
            Vec::new()
        } else {
            parts[5].split(',').map(ToString::to_string).collect()
        },
        parts[6].clone(),
    ))
}

fn schedule_phase_id(phase: &SchedulePhase) -> &'static str {
    match phase {
        SchedulePhase::HumanCommand => "human_command",
        SchedulePhase::NoHumanProcess => "no_human_process",
        SchedulePhase::DeferredProcess => "deferred_process",
        SchedulePhase::Replay => "replay",
    }
}

fn schedule_phase_from_id(value: &str) -> Option<SchedulePhase> {
    match value {
        "human_command" => Some(SchedulePhase::HumanCommand),
        "no_human_process" => Some(SchedulePhase::NoHumanProcess),
        "deferred_process" => Some(SchedulePhase::DeferredProcess),
        "replay" => Some(SchedulePhase::Replay),
        _ => None,
    }
}

fn serialize_source_id(source_id: &SchedulerSourceId) -> String {
    match source_id {
        SchedulerSourceId::Actor(id) => format!("actor:{}", id.as_str()),
        SchedulerSourceId::Controller(id) => format!("controller:{}", id.as_str()),
        SchedulerSourceId::Process(id) => format!("process:{}", id.as_str()),
    }
}

fn deserialize_source_id(value: &str) -> Result<SchedulerSourceId, EventEnvelopeParseError> {
    let (kind, value) = value
        .split_once(':')
        .ok_or(EventEnvelopeParseError::UnknownSchedulerSource)?;
    match kind {
        "actor" => Ok(SchedulerSourceId::Actor(ActorId::new(value)?)),
        "controller" => Ok(SchedulerSourceId::Controller(ControllerId::new(value)?)),
        "process" => Ok(SchedulerSourceId::Process(ProcessId::new(value)?)),
        _ => Err(EventEnvelopeParseError::UnknownSchedulerSource),
    }
}

fn serialize_cause(cause: &EventCause) -> String {
    match cause {
        EventCause::Event(id) => format!("event:{}", id.as_str()),
        EventCause::Proposal(id) => format!("proposal:{}", id.as_str()),
        EventCause::ValidationReport(id) => format!("validation_report:{}", id.as_str()),
        EventCause::Process(id) => format!("process:{}", id.as_str()),
    }
}

fn deserialize_cause(value: &str) -> Result<EventCause, EventEnvelopeParseError> {
    let (kind, value) = value
        .split_once(':')
        .ok_or(EventEnvelopeParseError::UnknownCause)?;
    match kind {
        "event" => Ok(EventCause::Event(EventId::new(value)?)),
        "proposal" => Ok(EventCause::Proposal(ProposalId::new(value)?)),
        "validation_report" => Ok(EventCause::ValidationReport(ValidationReportId::new(
            value,
        )?)),
        "process" => Ok(EventCause::Process(ProcessId::new(value)?)),
        _ => Err(EventEnvelopeParseError::UnknownCause),
    }
}

fn serialize_random_draw(draw: &RandomDrawRef) -> String {
    [draw.scope.clone(), draw.draw_id.clone(), draw.value.clone()]
        .into_iter()
        .map(|part| encode(&part))
        .collect::<Vec<_>>()
        .join(":")
}

fn deserialize_random_draw(value: &str) -> Result<RandomDrawRef, EventEnvelopeParseError> {
    let parts = value
        .split(':')
        .map(decode)
        .collect::<Result<Vec<_>, _>>()?;
    if parts.len() != 3 {
        return Err(EventEnvelopeParseError::InvalidTuple);
    }
    Ok(RandomDrawRef::new(&parts[0], &parts[1], &parts[2]))
}

fn serialize_payload(field: &PayloadField) -> String {
    format!("{}:{}", encode(&field.key), encode(&field.value))
}

fn deserialize_payload(value: &str) -> Result<PayloadField, EventEnvelopeParseError> {
    let (key, value) = value
        .split_once(':')
        .ok_or(EventEnvelopeParseError::InvalidTuple)?;
    Ok(PayloadField::new(decode(key)?, decode(value)?))
}

fn encode_opt(value: Option<&str>) -> String {
    value.map(encode).unwrap_or_default()
}

fn decode_opt(value: &str) -> Result<Option<String>, EventEnvelopeParseError> {
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(decode(value)?))
    }
}

fn encode_vec(values: &[String]) -> String {
    values
        .iter()
        .map(|value| encode(value))
        .collect::<Vec<_>>()
        .join(";")
}

fn decode_vec(value: &str) -> Result<Vec<String>, EventEnvelopeParseError> {
    if value.is_empty() {
        Ok(Vec::new())
    } else {
        value.split(';').map(decode).collect()
    }
}

fn encode(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn decode(value: &str) -> Result<String, EventEnvelopeParseError> {
    if !value.len().is_multiple_of(2) {
        return Err(EventEnvelopeParseError::BadHex);
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    let chars: Vec<_> = value.as_bytes().chunks_exact(2).collect();
    for chunk in chars {
        let hex = std::str::from_utf8(chunk).map_err(|_| EventEnvelopeParseError::BadHex)?;
        let byte = u8::from_str_radix(hex, 16).map_err(|_| EventEnvelopeParseError::BadHex)?;
        bytes.push(byte);
    }

    String::from_utf8(bytes).map_err(|_| EventEnvelopeParseError::InvalidUtf8)
}
