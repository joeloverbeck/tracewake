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
}
