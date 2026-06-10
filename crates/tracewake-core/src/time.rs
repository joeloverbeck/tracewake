use crate::state::NeedModelState;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimTick(u64);

impl SimTick {
    pub const ZERO: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub const fn advance_by(self, delta: u64) -> Self {
        Self(self.0 + delta)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassiveNeedDeltas {
    pub hunger_delta: i32,
    pub fatigue_delta: i32,
}

pub fn passive_awake_need_deltas(
    need_model: &NeedModelState,
    elapsed_ticks: u64,
) -> PassiveNeedDeltas {
    let elapsed = i32::try_from(elapsed_ticks).unwrap_or(i32::MAX);
    PassiveNeedDeltas {
        hunger_delta: elapsed.saturating_mul(need_model.awake_hunger_delta_per_tick),
        fatigue_delta: elapsed.saturating_mul(need_model.awake_fatigue_delta_per_tick),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_advance_is_deterministic() {
        let tick = SimTick::ZERO;

        assert_eq!(tick.value(), 0);
        assert_eq!(tick.next(), SimTick::new(1));
        assert_eq!(tick.advance_by(5), SimTick::new(5));
        assert_eq!(tick.advance_by(1).advance_by(1), SimTick::new(2));
    }

    #[test]
    fn passive_awake_need_deltas_are_deterministic_and_non_reducing() {
        assert_eq!(
            passive_awake_need_deltas(&NeedModelState::new(5, 3), 3),
            PassiveNeedDeltas {
                hunger_delta: 15,
                fatigue_delta: 9,
            }
        );
        assert!(passive_awake_need_deltas(&NeedModelState::new(5, 3), 10).hunger_delta >= 0);
        assert!(passive_awake_need_deltas(&NeedModelState::new(5, 3), 10).fatigue_delta >= 0);
    }
}
