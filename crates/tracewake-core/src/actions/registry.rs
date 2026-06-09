use std::collections::{BTreeMap, BTreeSet};

use crate::ids::ActionId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionEffect {
    QueryOnly,
    Move,
    Open,
    Close,
    Take,
    Place,
    Wait,
    CheckContainer,
    Sleep,
    Eat,
    Work,
    ContinueRoutine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionScope {
    Phase1,
    Phase2AHistorical,
    Phase3AHistorical,
}

impl ActionScope {
    pub const fn all_current() -> [Self; 3] {
        [
            Self::Phase1,
            Self::Phase2AHistorical,
            Self::Phase3AHistorical,
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionDefinition {
    pub action_id: ActionId,
    pub scope: ActionScope,
    pub effect: ActionEffect,
}

impl ActionDefinition {
    pub fn query_only(action_id: ActionId, scope: ActionScope) -> Self {
        Self {
            action_id,
            scope,
            effect: ActionEffect::QueryOnly,
        }
    }

    pub fn world_action(action_id: ActionId, scope: ActionScope, effect: ActionEffect) -> Self {
        Self {
            action_id,
            scope,
            effect,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionRegistry {
    definitions: BTreeMap<ActionId, ActionDefinition>,
    active_scopes: BTreeSet<ActionScope>,
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            definitions: BTreeMap::new(),
            active_scopes: ActionScope::all_current().into_iter().collect(),
        }
    }

    pub fn new_with_active_scopes(scopes: impl IntoIterator<Item = ActionScope>) -> Self {
        Self {
            definitions: BTreeMap::new(),
            active_scopes: scopes.into_iter().collect(),
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

    pub fn scope_is_active(&self, scope: ActionScope) -> bool {
        self.active_scopes.contains(&scope)
    }

    pub fn register_phase1_movement_open_close(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("move").unwrap(),
            ActionScope::Phase1,
            ActionEffect::Move,
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("open").unwrap(),
            ActionScope::Phase1,
            ActionEffect::Open,
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("close").unwrap(),
            ActionScope::Phase1,
            ActionEffect::Close,
        ));
    }

    pub fn register_phase1_take_place(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("take").unwrap(),
            ActionScope::Phase1,
            ActionEffect::Take,
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("place").unwrap(),
            ActionScope::Phase1,
            ActionEffect::Place,
        ));
    }

    pub fn register_phase1_inspect_wait(&mut self) {
        self.register(ActionDefinition::query_only(
            ActionId::new("look").unwrap(),
            ActionScope::Phase1,
        ));
        self.register(ActionDefinition::query_only(
            ActionId::new("inspect_place").unwrap(),
            ActionScope::Phase1,
        ));
        self.register(ActionDefinition::query_only(
            ActionId::new("inspect_entity").unwrap(),
            ActionScope::Phase1,
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("wait").unwrap(),
            ActionScope::Phase1,
            ActionEffect::Wait,
        ));
    }

    pub fn register_phase2a_epistemics(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("check_container").unwrap(),
            ActionScope::Phase2AHistorical,
            ActionEffect::CheckContainer,
        ));
        self.register(ActionDefinition::query_only(
            ActionId::new("truthful_accuse_probe").unwrap(),
            ActionScope::Phase2AHistorical,
        ));
    }

    pub fn register_phase3a_sleep(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("sleep").unwrap(),
            ActionScope::Phase3AHistorical,
            ActionEffect::Sleep,
        ));
    }

    pub fn register_phase3a_eat(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("eat").unwrap(),
            ActionScope::Phase3AHistorical,
            ActionEffect::Eat,
        ));
    }

    pub fn register_phase3a_work(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("work_block").unwrap(),
            ActionScope::Phase3AHistorical,
            ActionEffect::Work,
        ));
    }

    pub fn register_phase3a_continue_routine(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("continue_routine").unwrap(),
            ActionScope::Phase3AHistorical,
            ActionEffect::ContinueRoutine,
        ));
    }
}
