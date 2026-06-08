pub mod defs;
pub mod pipeline;
pub mod proposal;
pub mod registry;
pub mod report;

pub use pipeline::{
    run_pipeline, validate_proposal, PipelineContext, PipelineResult, PipelineStage,
    ProposalValidationContext,
};
pub use proposal::{Proposal, ProposalOrigin};
pub use registry::{ActionDefinition, ActionEffect, ActionRegistry};
pub use report::{CheckedFact, ReasonCode, ReportStatus, ValidationReport};
