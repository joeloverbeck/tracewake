use std::fmt;

use crate::ids::{ActionId, ActorId, RoutineExecutionId};

pub const NEED_MIN: u16 = 0;
pub const NEED_MAX: u16 = 1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NeedKind {
    Hunger,
    Fatigue,
    Safety,
}

impl NeedKind {
    pub const fn stable_id(self) -> &'static str {
        match self {
            NeedKind::Hunger => "hunger",
            NeedKind::Fatigue => "fatigue",
            NeedKind::Safety => "safety",
        }
    }

    fn parse(value: &str) -> Result<Self, NeedParseError> {
        match value {
            "hunger" => Ok(Self::Hunger),
            "fatigue" => Ok(Self::Fatigue),
            "safety" => Ok(Self::Safety),
            _ => Err(NeedParseError::InvalidNeedKind),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NeedBand {
    Comfortable,
    Rising,
    Urgent,
    Severe,
}

impl NeedBand {
    pub const fn stable_id(self) -> &'static str {
        match self {
            NeedBand::Comfortable => "comfortable",
            NeedBand::Rising => "rising",
            NeedBand::Urgent => "urgent",
            NeedBand::Severe => "severe",
        }
    }

    pub const fn for_value(value: u16) -> Self {
        match value {
            0..=249 => Self::Comfortable,
            250..=499 => Self::Rising,
            500..=749 => Self::Urgent,
            _ => Self::Severe,
        }
    }

    fn parse(value: &str) -> Result<Self, NeedParseError> {
        match value {
            "comfortable" => Ok(Self::Comfortable),
            "rising" => Ok(Self::Rising),
            "urgent" => Ok(Self::Urgent),
            "severe" => Ok(Self::Severe),
            _ => Err(NeedParseError::InvalidNeedBand),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThresholdDirection {
    IncreasingPressure,
    DecreasingPressure,
}

impl ThresholdDirection {
    pub const fn stable_id(self) -> &'static str {
        match self {
            ThresholdDirection::IncreasingPressure => "increasing_pressure",
            ThresholdDirection::DecreasingPressure => "decreasing_pressure",
        }
    }

    fn parse(value: &str) -> Result<Self, NeedParseError> {
        match value {
            "increasing_pressure" => Ok(Self::IncreasingPressure),
            "decreasing_pressure" => Ok(Self::DecreasingPressure),
            _ => Err(NeedParseError::InvalidThresholdDirection),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeedThresholdCrossing {
    pub from: NeedBand,
    pub to: NeedBand,
    pub direction: ThresholdDirection,
}

impl NeedThresholdCrossing {
    pub fn stable_id(&self) -> String {
        format!(
            "{}>{}:{}",
            self.from.stable_id(),
            self.to.stable_id(),
            self.direction.stable_id()
        )
    }

    fn parse(value: &str) -> Result<Option<Self>, NeedParseError> {
        if value == "none" {
            return Ok(None);
        }
        let (bands, direction) = value
            .split_once(':')
            .ok_or(NeedParseError::InvalidThresholdCrossing)?;
        let (from, to) = bands
            .split_once('>')
            .ok_or(NeedParseError::InvalidThresholdCrossing)?;
        Ok(Some(Self {
            from: NeedBand::parse(from)?,
            to: NeedBand::parse(to)?,
            direction: ThresholdDirection::parse(direction)?,
        }))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NeedChangeCause {
    FixtureInitial,
    TickDelta,
    ActionEffect(ActionId),
    RoutineEffect(RoutineExecutionId),
}

impl NeedChangeCause {
    pub fn stable_id(&self) -> String {
        match self {
            NeedChangeCause::FixtureInitial => "fixture_initial".to_string(),
            NeedChangeCause::TickDelta => "tick_delta".to_string(),
            NeedChangeCause::ActionEffect(action_id) => {
                format!("action_effect:{}", action_id.serialize_canonical())
            }
            NeedChangeCause::RoutineEffect(routine_execution_id) => {
                format!(
                    "routine_effect:{}",
                    routine_execution_id.serialize_canonical()
                )
            }
        }
    }

    fn parse(value: &str) -> Result<Self, NeedParseError> {
        if value == "fixture_initial" {
            return Ok(Self::FixtureInitial);
        }
        if value == "tick_delta" {
            return Ok(Self::TickDelta);
        }
        if let Some(action_id) = value.strip_prefix("action_effect:") {
            return Ok(Self::ActionEffect(
                ActionId::new(action_id).map_err(NeedParseError::InvalidId)?,
            ));
        }
        if let Some(routine_execution_id) = value.strip_prefix("routine_effect:") {
            return Ok(Self::RoutineEffect(
                RoutineExecutionId::new(routine_execution_id).map_err(NeedParseError::InvalidId)?,
            ));
        }
        Err(NeedParseError::InvalidCause)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeedState {
    kind: NeedKind,
    value: u16,
    last_change_cause: NeedChangeCause,
    last_threshold_crossing: Option<NeedThresholdCrossing>,
}

impl NeedState {
    pub fn initial(kind: NeedKind, value: i32, cause: NeedChangeCause) -> Self {
        Self {
            kind,
            value: clamp_need_value(value),
            last_change_cause: cause,
            last_threshold_crossing: None,
        }
    }

    pub const fn kind(&self) -> NeedKind {
        self.kind
    }

    pub const fn value(&self) -> u16 {
        self.value
    }

    pub const fn band(&self) -> NeedBand {
        NeedBand::for_value(self.value)
    }

    pub const fn last_change_cause(&self) -> &NeedChangeCause {
        &self.last_change_cause
    }

    pub const fn last_threshold_crossing(&self) -> Option<NeedThresholdCrossing> {
        self.last_threshold_crossing
    }

    pub fn last_change_source_label(&self) -> String {
        self.last_change_cause.stable_id()
    }

    pub fn apply_delta(
        &mut self,
        delta: i32,
        cause: NeedChangeCause,
    ) -> Option<NeedThresholdCrossing> {
        let previous = self.value;
        self.value = clamp_need_value(i32::from(previous) + delta);
        self.last_change_cause = cause;
        let crossing = Self::threshold_crossing(previous, self.value);
        if crossing.is_some() {
            self.last_threshold_crossing = crossing;
        }
        crossing
    }

    pub fn threshold_crossing(from_value: u16, to_value: u16) -> Option<NeedThresholdCrossing> {
        let from = NeedBand::for_value(from_value);
        let to = NeedBand::for_value(to_value);
        if from == to {
            return None;
        }

        let direction = if to_value > from_value {
            ThresholdDirection::IncreasingPressure
        } else {
            ThresholdDirection::DecreasingPressure
        };

        Some(NeedThresholdCrossing {
            from,
            to,
            direction,
        })
    }

    pub fn derive_pressure(
        &self,
        actor_id: ActorId,
        crossing: Option<NeedThresholdCrossing>,
        actor_known_explanation: impl Into<String>,
        debug_detail: impl Into<String>,
    ) -> NeedPressure {
        let band = self.band();
        NeedPressure {
            actor_id,
            need_kind: self.kind,
            value: self.value,
            band,
            threshold_crossing: crossing,
            source_ancestry: self.last_change_cause.clone(),
            interrupt_eligible: matches!(band, NeedBand::Urgent | NeedBand::Severe),
            actor_known_explanation: actor_known_explanation.into(),
            debug_detail: debug_detail.into(),
        }
    }

    pub fn serialize_canonical(&self) -> String {
        format!(
            "need_state_v2|{}|{:04}|{}|{}",
            self.kind.stable_id(),
            self.value,
            self.last_change_cause.stable_id(),
            self.last_threshold_crossing
                .as_ref()
                .map(NeedThresholdCrossing::stable_id)
                .unwrap_or_else(|| "none".to_string())
        )
    }

    pub fn serialize_canonical_bytes(&self) -> Vec<u8> {
        self.serialize_canonical().into_bytes()
    }

    pub fn deserialize_canonical(value: &[u8]) -> Result<Self, NeedParseError> {
        let value = std::str::from_utf8(value).map_err(|_| NeedParseError::InvalidUtf8)?;
        let mut fields = value.split('|');
        let version = fields.next().ok_or(NeedParseError::InvalidShape)?;
        let kind = fields.next().ok_or(NeedParseError::InvalidShape)?;
        let need_value = fields.next().ok_or(NeedParseError::InvalidShape)?;
        let cause = fields.next().ok_or(NeedParseError::InvalidShape)?;
        let crossing = fields.next().ok_or(NeedParseError::InvalidShape)?;
        if fields.next().is_some() || version != "need_state_v2" {
            return Err(NeedParseError::InvalidShape);
        }

        let need_value = need_value
            .parse::<u16>()
            .map_err(|_| NeedParseError::InvalidValue)?;
        if need_value > NEED_MAX {
            return Err(NeedParseError::InvalidValue);
        }

        Ok(Self {
            kind: NeedKind::parse(kind)?,
            value: need_value,
            last_change_cause: NeedChangeCause::parse(cause)?,
            last_threshold_crossing: NeedThresholdCrossing::parse(crossing)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NeedPressure {
    pub actor_id: ActorId,
    pub need_kind: NeedKind,
    pub value: u16,
    pub band: NeedBand,
    pub threshold_crossing: Option<NeedThresholdCrossing>,
    pub source_ancestry: NeedChangeCause,
    pub interrupt_eligible: bool,
    pub actor_known_explanation: String,
    pub debug_detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NeedParseError {
    InvalidUtf8,
    InvalidShape,
    InvalidNeedKind,
    InvalidNeedBand,
    InvalidThresholdDirection,
    InvalidThresholdCrossing,
    InvalidValue,
    InvalidCause,
    InvalidId(crate::ids::IdError),
}

impl fmt::Display for NeedParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeedParseError::InvalidUtf8 => write!(f, "canonical need bytes must be UTF-8"),
            NeedParseError::InvalidShape => write!(f, "invalid canonical need shape"),
            NeedParseError::InvalidNeedKind => write!(f, "invalid need kind"),
            NeedParseError::InvalidNeedBand => write!(f, "invalid need band"),
            NeedParseError::InvalidThresholdDirection => {
                write!(f, "invalid need threshold direction")
            }
            NeedParseError::InvalidThresholdCrossing => {
                write!(f, "invalid need threshold crossing")
            }
            NeedParseError::InvalidValue => write!(f, "invalid need value"),
            NeedParseError::InvalidCause => write!(f, "invalid need change cause"),
            NeedParseError::InvalidId(err) => write!(f, "invalid need cause ID: {err}"),
        }
    }
}

impl std::error::Error for NeedParseError {}

fn clamp_need_value(value: i32) -> u16 {
    value.clamp(i32::from(NEED_MIN), i32::from(NEED_MAX)) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn need_state_round_trips_through_canonical_bytes() {
        let state = NeedState::initial(
            NeedKind::Hunger,
            250,
            NeedChangeCause::ActionEffect(ActionId::new("eat.breakfast").unwrap()),
        );

        let bytes = state.serialize_canonical_bytes();
        let round_tripped = NeedState::deserialize_canonical(&bytes).unwrap();

        assert_eq!(round_tripped, state);
        assert_eq!(round_tripped.serialize_canonical_bytes(), bytes);
    }

    #[test]
    fn need_bands_match_phase_3a_boundaries() {
        assert_eq!(
            NeedState::initial(NeedKind::Hunger, 249, NeedChangeCause::FixtureInitial).band(),
            NeedBand::Comfortable
        );
        assert_eq!(
            NeedState::initial(NeedKind::Hunger, 250, NeedChangeCause::FixtureInitial).band(),
            NeedBand::Rising
        );
        assert_eq!(
            NeedState::initial(NeedKind::Hunger, 499, NeedChangeCause::FixtureInitial).band(),
            NeedBand::Rising
        );
        assert_eq!(
            NeedState::initial(NeedKind::Hunger, 500, NeedChangeCause::FixtureInitial).band(),
            NeedBand::Urgent
        );
        assert_eq!(
            NeedState::initial(NeedKind::Hunger, 749, NeedChangeCause::FixtureInitial).band(),
            NeedBand::Urgent
        );
        assert_eq!(
            NeedState::initial(NeedKind::Hunger, 750, NeedChangeCause::FixtureInitial).band(),
            NeedBand::Severe
        );
    }

    #[test]
    fn apply_delta_clamps_without_underflow_or_overflow() {
        let mut low = NeedState::initial(NeedKind::Fatigue, 260, NeedChangeCause::FixtureInitial);
        assert_eq!(
            low.apply_delta(-2000, NeedChangeCause::TickDelta)
                .unwrap()
                .to,
            NeedBand::Comfortable
        );
        assert_eq!(low.value(), 0);

        let mut high = NeedState::initial(NeedKind::Safety, 740, NeedChangeCause::FixtureInitial);
        assert_eq!(
            high.apply_delta(2000, NeedChangeCause::TickDelta)
                .unwrap()
                .to,
            NeedBand::Severe
        );
        assert_eq!(high.value(), 1000);

        for start in 0..=1000 {
            for delta in [-2500, -1001, -1, 0, 1, 1001, 2500] {
                let mut state =
                    NeedState::initial(NeedKind::Hunger, start, NeedChangeCause::FixtureInitial);
                state.apply_delta(delta, NeedChangeCause::TickDelta);
                assert!(state.value() <= NEED_MAX);
            }
        }
    }

    #[test]
    fn threshold_crossing_reports_band_transitions() {
        assert_eq!(
            NeedState::threshold_crossing(249, 250),
            Some(NeedThresholdCrossing {
                from: NeedBand::Comfortable,
                to: NeedBand::Rising,
                direction: ThresholdDirection::IncreasingPressure,
            })
        );
        assert_eq!(
            NeedState::threshold_crossing(750, 499),
            Some(NeedThresholdCrossing {
                from: NeedBand::Severe,
                to: NeedBand::Rising,
                direction: ThresholdDirection::DecreasingPressure,
            })
        );
        assert_eq!(NeedState::threshold_crossing(500, 749), None);
    }

    #[test]
    fn pressure_is_derived_from_state_and_not_stored_authority() {
        let mut state = NeedState::initial(NeedKind::Hunger, 490, NeedChangeCause::FixtureInitial);
        let crossing = state.apply_delta(25, NeedChangeCause::TickDelta);

        let pressure = state.derive_pressure(
            ActorId::new("actor_mara").unwrap(),
            crossing,
            "I am getting hungry",
            "tick delta crossed hunger into urgent",
        );

        assert_eq!(pressure.need_kind, NeedKind::Hunger);
        assert_eq!(pressure.value, 515);
        assert_eq!(pressure.band, NeedBand::Urgent);
        assert!(pressure.interrupt_eligible);
        assert_eq!(pressure.source_ancestry, NeedChangeCause::TickDelta);
        assert_eq!(
            pressure.threshold_crossing.unwrap().direction,
            ThresholdDirection::IncreasingPressure
        );
    }

    #[test]
    fn last_change_source_label_is_deterministic_for_debug_and_replay() {
        let mut state = NeedState::initial(NeedKind::Hunger, 620, NeedChangeCause::FixtureInitial);
        state.apply_delta(
            -240,
            NeedChangeCause::ActionEffect(ActionId::new("eat").unwrap()),
        );

        assert_eq!(state.last_change_source_label(), "action_effect:eat");
        assert_eq!(
            NeedState::deserialize_canonical(&state.serialize_canonical_bytes())
                .unwrap()
                .last_change_source_label(),
            "action_effect:eat"
        );
    }
}
