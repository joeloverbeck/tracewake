use std::collections::BTreeMap;

use crate::ids::{ActionId, ActorId, ProposalId, ViewModelId};
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
        }
    }
}
