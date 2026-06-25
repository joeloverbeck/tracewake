use crate::actions::{PipelineResult, ValidationReport};
use crate::debug_capability::DebugCapability;
use crate::events::EventEnvelope;
use crate::ids::EventId;
use crate::scheduler::no_human::NoHumanDayReport;
use crate::scheduler::{AdvanceUntilResult, WorldAdvanceResult};
use crate::time::SimTick;

/// Immutable runtime receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReceipt {
    kind: RuntimeReceiptKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeReceiptKind {
    OneTickAdvanced(WorldAdvanceResult),
    ActionSubmitted(RuntimeActionReceipt),
    Continued(AdvanceUntilResult),
    NoHumanDay(NoHumanDayReport),
    Embodied(EmbodiedRuntimeReceipt),
    Debug(DebugRuntimeReceipt),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeActionReceipt {
    pub report: ValidationReport,
    pub appended_events: Vec<EventEnvelope>,
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
            kind: RuntimeReceiptKind::Continued(result),
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
        capability: DebugCapability,
        prior_tick: SimTick,
        resulting_tick: SimTick,
        event_ids: Vec<EventId>,
        stop_reason: Option<String>,
    ) -> Self {
        Self {
            capability,
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
        let receipt = DebugRuntimeReceipt::new(
            DebugCapability::mint(),
            SimTick::ZERO,
            SimTick::new(1),
            Vec::new(),
            Some("test".to_string()),
        );

        assert!(receipt.debug_only());
        assert_eq!(receipt.prior_tick(), SimTick::ZERO);
        assert_eq!(receipt.resulting_tick(), SimTick::new(1));
        assert_eq!(receipt.stop_reason(), Some("test"));
    }
}
