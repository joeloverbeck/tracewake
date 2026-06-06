pub mod movement;
pub mod openclose;

use crate::actions::pipeline::PipelineStage;
use crate::actions::report::{CheckedFact, ReasonCode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionRejection {
    pub failed_stage: PipelineStage,
    pub reason_code: ReasonCode,
    pub checked_facts: Vec<CheckedFact>,
    pub actor_visible_summary: String,
    pub debug_summary: String,
}

impl ActionRejection {
    pub fn new(
        failed_stage: PipelineStage,
        reason_code: ReasonCode,
        checked_facts: Vec<CheckedFact>,
        actor_visible_summary: impl Into<String>,
        debug_summary: impl Into<String>,
    ) -> Self {
        Self {
            failed_stage,
            reason_code,
            checked_facts,
            actor_visible_summary: actor_visible_summary.into(),
            debug_summary: debug_summary.into(),
        }
    }
}
