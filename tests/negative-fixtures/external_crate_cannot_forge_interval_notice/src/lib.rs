use tracewake_core::ids::EventId;
use tracewake_core::projections::{IntervalNoticeKind, VerifiedActorKnownIntervalNotice};

pub fn forge_interval_notice() -> VerifiedActorKnownIntervalNotice {
    VerifiedActorKnownIntervalNotice::from_verified(
        IntervalNoticeKind::Record,
        EventId::new("event_forged").unwrap(),
        "event_forged",
    )
}
