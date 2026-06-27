use crate::debug_capability::DebugSessionAuthority;
use crate::ids::{ActorId, ControllerId};
use crate::scheduler::WorldAdvanceOrigin;
use crate::state::ControllerMode;
use crate::view_models::{EmbodiedViewModel, SemanticActionEntry};

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
    SubmitSemanticAction {
        controller_id: ControllerId,
        actor_id: ActorId,
        entry: SemanticActionEntry,
        source_view: Box<EmbodiedViewModel>,
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
        mode: ControllerMode,
    },
    DetachController {
        controller_id: ControllerId,
    },
    RunNoHumanDay {
        _authority: DebugSessionAuthority,
    },
    RebuildFromReplaySeed,
    EmbodiedView {
        actor_id: ActorId,
    },
    DebugView {
        _authority: DebugSessionAuthority,
    },
}

impl RuntimeCommand {
    pub fn submit_semantic_action(
        controller_id: ControllerId,
        actor_id: ActorId,
        entry: SemanticActionEntry,
        source_view: EmbodiedViewModel,
    ) -> Self {
        Self {
            kind: RuntimeCommandKind::SubmitSemanticAction {
                controller_id,
                actor_id,
                entry,
                source_view: Box::new(source_view),
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
                mode: ControllerMode::Embodied,
            },
        }
    }

    pub fn bind_debug_controller(controller_id: ControllerId, actor_id: ActorId) -> Self {
        Self {
            kind: RuntimeCommandKind::BindController {
                controller_id,
                actor_id,
                mode: ControllerMode::Debug,
            },
        }
    }

    pub fn detach_controller(controller_id: ControllerId) -> Self {
        Self {
            kind: RuntimeCommandKind::DetachController { controller_id },
        }
    }

    pub fn run_no_human_day(authority: DebugSessionAuthority) -> Self {
        Self {
            kind: RuntimeCommandKind::RunNoHumanDay {
                _authority: authority,
            },
        }
    }

    pub fn rebuild_from_replay_seed() -> Self {
        Self {
            kind: RuntimeCommandKind::RebuildFromReplaySeed,
        }
    }

    pub fn embodied_view(actor_id: ActorId) -> Self {
        Self {
            kind: RuntimeCommandKind::EmbodiedView { actor_id },
        }
    }

    pub fn debug_view(authority: DebugSessionAuthority) -> Self {
        Self {
            kind: RuntimeCommandKind::DebugView {
                _authority: authority,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_constructors_do_not_accept_runtime_authority() {
        let command = RuntimeCommand::one_tick_wait(WorldAdvanceOrigin::Controller(
            ControllerId::new("controller_human").unwrap(),
        ));

        match command.kind {
            RuntimeCommandKind::OneTickWait { .. } => {}
            _ => panic!("one_tick_wait constructor returned wrong command kind"),
        }
    }
}
