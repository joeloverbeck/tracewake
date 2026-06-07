//! Agent cognition substrate for ordinary-life simulation.

pub mod need;

pub use need::{
    NeedBand, NeedChangeCause, NeedKind, NeedParseError, NeedPressure, NeedState,
    NeedThresholdCrossing, ThresholdDirection,
};
