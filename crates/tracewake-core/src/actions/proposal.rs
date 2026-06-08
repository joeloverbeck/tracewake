use std::collections::BTreeMap;

use crate::checksum::HolderKnownContextHash;
use crate::ids::{
    ActionId, ActorId, HolderKnownContextId, ProposalId, SemanticActionId, ViewModelId,
};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalOrigin {
    Human,
    Test,
    Scheduler,
    Agent,
    Debug,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub proposal_id: ProposalId,
    pub origin: ProposalOrigin,
    pub actor_id: Option<ActorId>,
    pub action_id: ActionId,
    pub target_ids: Vec<String>,
    pub parameters: BTreeMap<String, String>,
    pub requested_tick: SimTick,
    pub source_view_model_id: Option<ViewModelId>,
    pub source: Option<ProposalSource>,
}

impl Proposal {
    pub fn new(
        proposal_id: ProposalId,
        origin: ProposalOrigin,
        actor_id: Option<ActorId>,
        action_id: ActionId,
        requested_tick: SimTick,
    ) -> Self {
        Self {
            proposal_id,
            origin,
            actor_id,
            action_id,
            target_ids: Vec::new(),
            parameters: BTreeMap::new(),
            requested_tick,
            source_view_model_id: None,
            source: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalSource {
    TuiSemanticAction(ProposalSourceContext),
    Scheduler,
    Agent,
    Test,
    Debug,
}

impl ProposalSource {
    pub const fn stable_id(&self) -> &'static str {
        match self {
            Self::TuiSemanticAction(_) => "tui_semantic_action",
            Self::Scheduler => "scheduler",
            Self::Agent => "agent",
            Self::Test => "test",
            Self::Debug => "debug",
        }
    }

    pub const fn tui_context(&self) -> Option<&ProposalSourceContext> {
        match self {
            Self::TuiSemanticAction(context) => Some(context),
            Self::Scheduler | Self::Agent | Self::Test | Self::Debug => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProposalSourceContext {
    pub source_view_model_id: ViewModelId,
    pub holder_known_context_id: HolderKnownContextId,
    pub holder_known_context_hash: HolderKnownContextHash,
    pub holder_known_context_frontier: u64,
    pub context_tick: SimTick,
    pub actor_id: ActorId,
    pub semantic_action_id: SemanticActionId,
    pub provenance_ancestry: Vec<String>,
}
