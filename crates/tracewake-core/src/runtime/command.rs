use crate::actions::Proposal;
use crate::debug_capability::DebugCapability;
use crate::ids::{ActorId, ControllerId};
use crate::scheduler::WorldAdvanceOrigin;

/// Closed runtime command token.
///
/// Clients can request semantic runtime operations through constructors, but
/// cannot inject scheduler state, proposal sequences, or choreography flags.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeCommand {
    pub(crate) kind: RuntimeCommandKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RuntimeCommandKind {
    SubmitProposal {
        controller_id: ControllerId,
        proposal: Proposal,
    },
    OneTickWait {
        origin: WorldAdvanceOrigin,
    },
    ContinueUntil {
        controller_id: ControllerId,
        possessed_actor_id: ActorId,
        max_ticks: u64,
    },
    BindController {
        controller_id: ControllerId,
        actor_id: ActorId,
    },
    DetachController {
        controller_id: ControllerId,
    },
    RunNoHumanDay {
        _capability: DebugCapability,
    },
    RebuildFromReplaySeed,
    RefreshActorCurrentPlacePerception {
        actor_id: ActorId,
    },
    EmbodiedView {
        actor_id: ActorId,
    },
    DebugView {
        _capability: DebugCapability,
    },
}

impl RuntimeCommand {
    pub fn submit_proposal(controller_id: ControllerId, proposal: Proposal) -> Self {
        Self {
            kind: RuntimeCommandKind::SubmitProposal {
                controller_id,
                proposal,
            },
        }
    }

    pub fn one_tick_wait(origin: WorldAdvanceOrigin) -> Self {
        Self {
            kind: RuntimeCommandKind::OneTickWait { origin },
        }
    }

    pub fn continue_until(
        controller_id: ControllerId,
        possessed_actor_id: ActorId,
        max_ticks: u64,
    ) -> Self {
        Self {
            kind: RuntimeCommandKind::ContinueUntil {
                controller_id,
                possessed_actor_id,
                max_ticks,
            },
        }
    }

    pub fn bind_controller(controller_id: ControllerId, actor_id: ActorId) -> Self {
        Self {
            kind: RuntimeCommandKind::BindController {
                controller_id,
                actor_id,
            },
        }
    }

    pub fn detach_controller(controller_id: ControllerId) -> Self {
        Self {
            kind: RuntimeCommandKind::DetachController { controller_id },
        }
    }

    pub fn run_no_human_day(capability: DebugCapability) -> Self {
        Self {
            kind: RuntimeCommandKind::RunNoHumanDay {
                _capability: capability,
            },
        }
    }

    pub fn rebuild_from_replay_seed() -> Self {
        Self {
            kind: RuntimeCommandKind::RebuildFromReplaySeed,
        }
    }

    pub fn refresh_actor_current_place_perception(actor_id: ActorId) -> Self {
        Self {
            kind: RuntimeCommandKind::RefreshActorCurrentPlacePerception { actor_id },
        }
    }

    pub fn embodied_view(actor_id: ActorId) -> Self {
        Self {
            kind: RuntimeCommandKind::EmbodiedView { actor_id },
        }
    }

    pub fn debug_view(capability: DebugCapability) -> Self {
        Self {
            kind: RuntimeCommandKind::DebugView {
                _capability: capability,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{ActionId, ProposalId};
    use crate::time::SimTick;

    #[test]
    fn command_constructors_do_not_accept_runtime_authority() {
        let controller_id = ControllerId::new("controller_human").unwrap();
        let actor_id = ActorId::new("actor_mara").unwrap();
        let proposal = Proposal::new(
            ProposalId::new("proposal_wait").unwrap(),
            crate::actions::ProposalOrigin::Human,
            Some(actor_id),
            ActionId::new("action_wait").unwrap(),
            SimTick::ZERO,
        );

        let command = RuntimeCommand::submit_proposal(controller_id, proposal);

        match command.kind {
            RuntimeCommandKind::SubmitProposal { .. } => {}
            _ => panic!("submit_proposal constructor returned wrong command kind"),
        }
    }
}
