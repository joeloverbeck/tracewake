use std::collections::BTreeMap;

use crate::ids::ActionId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionEffect {
    QueryOnly,
    WorldMutationDeferred,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionDefinition {
    pub action_id: ActionId,
    pub phase1_implemented: bool,
    pub effect: ActionEffect,
}

impl ActionDefinition {
    pub fn query_only(action_id: ActionId) -> Self {
        Self {
            action_id,
            phase1_implemented: true,
            effect: ActionEffect::QueryOnly,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActionRegistry {
    definitions: BTreeMap<ActionId, ActionDefinition>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, definition: ActionDefinition) {
        self.definitions
            .insert(definition.action_id.clone(), definition);
    }

    pub fn get(&self, action_id: &ActionId) -> Option<&ActionDefinition> {
        self.definitions.get(action_id)
    }

    pub fn definitions(&self) -> impl Iterator<Item = &ActionDefinition> {
        self.definitions.values()
    }
}
