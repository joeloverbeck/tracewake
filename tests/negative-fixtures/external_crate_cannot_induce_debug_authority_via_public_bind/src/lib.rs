use tracewake_core::ids::{ActorId, ControllerId};
use tracewake_core::runtime::RuntimeCommand;

pub fn induce_debug_authority_with_public_bind() {
    let _command = RuntimeCommand::bind_debug_controller(
        ControllerId::new("controller_external").unwrap(),
        ActorId::new("actor_external").unwrap(),
    );
}
