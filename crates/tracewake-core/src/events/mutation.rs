#[derive(Clone, Copy, Debug)]
pub struct WorldMutationCapability {
    _private: (),
}

#[derive(Clone, Copy, Debug)]
pub struct AgentMutationCapability {
    _private: (),
}

impl WorldMutationCapability {
    pub(in crate::events) fn mint() -> Self {
        Self { _private: () }
    }
}

impl AgentMutationCapability {
    pub(in crate::events) fn mint() -> Self {
        Self { _private: () }
    }
}
