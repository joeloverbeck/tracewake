use crate::actions::{PipelineResult, ValidationReport};
use crate::debug_capability::DebugCapability;
use crate::ids::EventId;
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
    ProposalSubmitted(PipelineResult),
    Continued(AdvanceUntilResult),
    Embodied(EmbodiedRuntimeReceipt),
    Debug(DebugRuntimeReceipt),
    Rejected(RuntimeRejectionReceipt),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeRejectionReceipt {
    report: ValidationReport,
}

impl RuntimeReceipt {
    pub(crate) fn one_tick_advanced(result: WorldAdvanceResult) -> Self {
        Self {
            kind: RuntimeReceiptKind::OneTickAdvanced(result),
        }
    }

    pub(crate) fn proposal_submitted(result: PipelineResult) -> Self {
        Self {
            kind: RuntimeReceiptKind::ProposalSubmitted(result),
        }
    }

    pub(crate) fn continued(result: AdvanceUntilResult) -> Self {
        Self {
            kind: RuntimeReceiptKind::Continued(result),
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

    pub(crate) fn rejected(report: ValidationReport) -> Self {
        Self {
            kind: RuntimeReceiptKind::Rejected(RuntimeRejectionReceipt { report }),
        }
    }

    pub fn kind(&self) -> &RuntimeReceiptKind {
        &self.kind
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

impl RuntimeRejectionReceipt {
    pub fn report(&self) -> &ValidationReport {
        &self.report
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
