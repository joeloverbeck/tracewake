use crate::actions::{PipelineResult, ValidationReport};
use crate::debug_capability::{DebugCapability, DebugSessionAuthority};
use crate::events::EventEnvelope;
use crate::ids::EventId;
use crate::scheduler::no_human::NoHumanDayReport;
use crate::scheduler::{
    ActorStepSummary, AdvanceUntilResult, WorldAdvanceResult, WorldStepDueWorkSummary,
};
use crate::time::SimTick;
use crate::view_models::TypedActorKnownIntervalSummary;

/// Immutable runtime receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReceipt {
    kind: RuntimeReceiptKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(
    clippy::large_enum_variant,
    reason = "Runtime receipts preserve existing public enum ergonomics; boxing action receipts is a separate API change."
)]
pub enum RuntimeReceiptKind {
    OneTickAdvanced(OneTickRuntimeReceipt),
    ActionSubmitted(RuntimeActionReceipt),
    Continued(ContinuedRuntimeReceipt),
    NoHumanDay(NoHumanDayReport),
    Embodied(EmbodiedRuntimeReceipt),
    Debug(DebugRuntimeReceipt),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeActionReceipt {
    pub report: ValidationReport,
    pub appended_events: Vec<EventEnvelope>,
}

/// Actor-legible one-tick wait product. It mirrors the continuation receipt:
/// visible progress and actor-known interval summary are exposed, while exact
/// scheduler ticks, event IDs, due-work queues, ancestry, and trace IDs stay out
/// of the normal receipt boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OneTickRuntimeReceipt {
    advanced: bool,
    appended_event_count: usize,
    actor_known_interval_summary: Option<TypedActorKnownIntervalSummary>,
}

/// Actor-legible continuation product. It carries whether visible progress
/// occurred and the core-built actor-known interval summary, but not exact
/// scheduler ticks, frontiers, stop reasons, or replay event identifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuedRuntimeReceipt {
    advanced: bool,
    appended_event_count: usize,
    actor_known_interval_summary: Option<TypedActorKnownIntervalSummary>,
}

/// Actor-legible runtime product. It intentionally carries qualitative text
/// and provenance only, not exact ticks/frontiers or debug stop reasons.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbodiedRuntimeReceipt {
    actor_visible_summary: String,
    provenance: Vec<String>,
}

/// Privileged non-diegetic runtime product.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugRuntimeReceipt {
    capability: DebugCapability,
    prior_tick: SimTick,
    resulting_tick: SimTick,
    event_ids: Vec<EventId>,
    due_work_summary: Option<WorldStepDueWorkSummary>,
    actor_step_summaries: Vec<ActorStepSummary>,
    stop_reason: Option<String>,
}

impl RuntimeReceipt {
    pub(crate) fn one_tick_advanced(result: WorldAdvanceResult) -> Self {
        Self {
            kind: RuntimeReceiptKind::OneTickAdvanced(
                OneTickRuntimeReceipt::from_world_advance_result(result),
            ),
        }
    }

    pub(crate) fn action_submitted(result: RuntimeActionReceipt) -> Self {
        Self {
            kind: RuntimeReceiptKind::ActionSubmitted(result),
        }
    }

    pub(crate) fn continued(result: AdvanceUntilResult) -> Self {
        Self {
            kind: RuntimeReceiptKind::Continued(
                ContinuedRuntimeReceipt::from_advance_until_result(result),
            ),
        }
    }

    pub(crate) fn no_human_day(result: NoHumanDayReport) -> Self {
        Self {
            kind: RuntimeReceiptKind::NoHumanDay(result),
        }
    }

    pub(crate) fn embodied(receipt: EmbodiedRuntimeReceipt) -> Self {
        Self {
            kind: RuntimeReceiptKind::Embodied(receipt),
        }
    }

    pub(crate) fn debug(receipt: DebugRuntimeReceipt) -> Self {
        Self {
            kind: RuntimeReceiptKind::Debug(receipt),
        }
    }

    pub fn kind(&self) -> &RuntimeReceiptKind {
        &self.kind
    }

    pub fn into_action_receipt(self) -> Option<RuntimeActionReceipt> {
        match self.kind {
            RuntimeReceiptKind::ActionSubmitted(receipt) => Some(receipt),
            _ => None,
        }
    }

    pub fn into_no_human_day_report(self) -> Option<NoHumanDayReport> {
        match self.kind {
            RuntimeReceiptKind::NoHumanDay(report) => Some(report),
            _ => None,
        }
    }
}

impl From<PipelineResult> for RuntimeActionReceipt {
    fn from(value: PipelineResult) -> Self {
        Self {
            report: value.report,
            appended_events: value.appended_events,
        }
    }
}

impl OneTickRuntimeReceipt {
    fn from_world_advance_result(result: WorldAdvanceResult) -> Self {
        Self {
            advanced: result.resulting_tick > result.prior_tick,
            appended_event_count: result.appended_event_ids.len(),
            actor_known_interval_summary: result
                .actor_known_interval_delta
                .map(TypedActorKnownIntervalSummary::from_actor_known_delta),
        }
    }

    pub fn advanced(&self) -> bool {
        self.advanced
    }

    pub fn appended_event_count(&self) -> usize {
        self.appended_event_count
    }

    pub fn actor_known_interval_summary(&self) -> Option<&TypedActorKnownIntervalSummary> {
        self.actor_known_interval_summary.as_ref()
    }
}

impl ContinuedRuntimeReceipt {
    fn from_advance_until_result(result: AdvanceUntilResult) -> Self {
        Self {
            advanced: result.ticks_advanced > 0,
            appended_event_count: result.appended_event_ids.len(),
            actor_known_interval_summary: result
                .actor_known_interval_delta
                .map(TypedActorKnownIntervalSummary::from_actor_known_delta),
        }
    }

    pub fn advanced(&self) -> bool {
        self.advanced
    }

    pub fn appended_event_count(&self) -> usize {
        self.appended_event_count
    }

    pub fn actor_known_interval_summary(&self) -> Option<&TypedActorKnownIntervalSummary> {
        self.actor_known_interval_summary.as_ref()
    }
}

impl EmbodiedRuntimeReceipt {
    pub(crate) fn new(actor_visible_summary: impl Into<String>, provenance: Vec<String>) -> Self {
        Self {
            actor_visible_summary: actor_visible_summary.into(),
            provenance,
        }
    }

    pub fn actor_visible_summary(&self) -> &str {
        &self.actor_visible_summary
    }

    pub fn provenance(&self) -> &[String] {
        &self.provenance
    }
}

impl DebugRuntimeReceipt {
    pub(crate) fn new(
        authority: &DebugSessionAuthority,
        prior_tick: SimTick,
        resulting_tick: SimTick,
        event_ids: Vec<EventId>,
        stop_reason: Option<String>,
    ) -> Self {
        Self {
            capability: authority.capability(),
            prior_tick,
            resulting_tick,
            event_ids,
            due_work_summary: None,
            actor_step_summaries: Vec::new(),
            stop_reason,
        }
    }

    pub(crate) fn from_world_advance_result(
        authority: &DebugSessionAuthority,
        result: WorldAdvanceResult,
        stop_reason: Option<String>,
    ) -> Self {
        Self {
            capability: authority.capability(),
            prior_tick: result.prior_tick,
            resulting_tick: result.resulting_tick,
            event_ids: result.appended_event_ids,
            due_work_summary: Some(result.due_work_summary),
            actor_step_summaries: result.actor_step_summaries,
            stop_reason,
        }
    }

    pub fn debug_only(&self) -> bool {
        self.capability.debug_only()
    }

    pub fn prior_tick(&self) -> SimTick {
        self.prior_tick
    }

    pub fn resulting_tick(&self) -> SimTick {
        self.resulting_tick
    }

    pub fn event_ids(&self) -> &[EventId] {
        &self.event_ids
    }

    pub fn due_work_summary(&self) -> Option<&WorldStepDueWorkSummary> {
        self.due_work_summary.as_ref()
    }

    pub fn actor_step_summaries(&self) -> &[ActorStepSummary] {
        &self.actor_step_summaries
    }

    pub fn stop_reason(&self) -> Option<&str> {
        self.stop_reason.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::AdvanceUntilStopReason;

    #[test]
    fn embodied_receipt_exposes_no_exact_temporal_authority() {
        let receipt = EmbodiedRuntimeReceipt::new(
            "The world changes around you.",
            vec!["event:actor_visible_interval".to_string()],
        );

        assert_eq!(
            receipt.actor_visible_summary(),
            "The world changes around you."
        );
        assert_eq!(receipt.provenance(), ["event:actor_visible_interval"]);
    }

    #[test]
    fn continued_receipt_derives_progress_and_event_count_from_runtime_result() {
        let stationary_with_events =
            ContinuedRuntimeReceipt::from_advance_until_result(AdvanceUntilResult {
                start_tick: SimTick::new(4),
                stop_tick: SimTick::new(4),
                ticks_advanced: 0,
                stop_reason: AdvanceUntilStopReason::UserPausedBeforeNextTick,
                appended_event_ids: vec![
                    EventId::new("event.continue.marker.one").unwrap(),
                    EventId::new("event.continue.marker.two").unwrap(),
                ],
                actor_known_interval_delta: None,
            });

        assert!(!stationary_with_events.advanced());
        assert_eq!(stationary_with_events.appended_event_count(), 2);
        assert!(stationary_with_events
            .actor_known_interval_summary()
            .is_none());

        let advanced_without_events =
            ContinuedRuntimeReceipt::from_advance_until_result(AdvanceUntilResult {
                start_tick: SimTick::new(4),
                stop_tick: SimTick::new(5),
                ticks_advanced: 1,
                stop_reason: AdvanceUntilStopReason::ControllerSafetyBound,
                appended_event_ids: Vec::new(),
                actor_known_interval_delta: None,
            });

        assert!(advanced_without_events.advanced());
        assert_eq!(advanced_without_events.appended_event_count(), 0);
    }

    #[test]
    fn debug_receipt_is_capability_gated() {
        let event_ids = vec![
            EventId::new("event.debug.one").unwrap(),
            EventId::new("event.debug.two").unwrap(),
        ];
        let authority = DebugSessionAuthority::mint();
        let receipt = DebugRuntimeReceipt::new(
            &authority,
            SimTick::new(7),
            SimTick::new(9),
            event_ids.clone(),
            Some("test".to_string()),
        );

        assert!(receipt.debug_only());
        assert_eq!(receipt.prior_tick(), SimTick::new(7));
        assert_eq!(receipt.resulting_tick(), SimTick::new(9));
        assert_eq!(receipt.event_ids(), event_ids.as_slice());
        assert_eq!(receipt.stop_reason(), Some("test"));

        let non_debug = DebugRuntimeReceipt {
            capability: DebugCapability::test_non_debug(),
            prior_tick: SimTick::new(7),
            resulting_tick: SimTick::new(9),
            event_ids,
            due_work_summary: None,
            actor_step_summaries: Vec::new(),
            stop_reason: Some("test".to_string()),
        };
        assert!(!non_debug.debug_only());
    }
}
