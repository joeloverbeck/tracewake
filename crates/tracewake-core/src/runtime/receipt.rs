use crate::actions::{PipelineResult, ValidationReport};
use crate::debug_capability::{DebugCapability, DebugSessionAuthority};
use crate::events::EventEnvelope;
use crate::ids::EventId;
use crate::scheduler::no_human::NoHumanDayReport;
use crate::scheduler::{AdvanceUntilResult, WorldAdvanceResult};
use crate::time::SimTick;
use crate::view_models::TypedActorKnownIntervalSummary;

/// Immutable runtime receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReceipt {
    kind: RuntimeReceiptKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeReceiptKind {
    OneTickAdvanced(WorldAdvanceResult),
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
    stop_reason: Option<String>,
}

impl RuntimeReceipt {
    pub(crate) fn one_tick_advanced(result: WorldAdvanceResult) -> Self {
        Self {
            kind: RuntimeReceiptKind::OneTickAdvanced(result),
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

    pub fn stop_reason(&self) -> Option<&str> {
        self.stop_reason.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            stop_reason: Some("test".to_string()),
        };
        assert!(!non_debug.debug_only());
    }
}
