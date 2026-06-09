use tracewake_core::events::mutation::{AgentMutationCapability, WorldMutationCapability};

pub fn forge_world_capability() -> WorldMutationCapability {
    WorldMutationCapability { _private: () }
}

pub fn forge_agent_capability() -> AgentMutationCapability {
    AgentMutationCapability { _private: () }
}
