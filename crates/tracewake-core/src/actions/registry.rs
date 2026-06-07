use std::collections::BTreeMap;

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

    pub fn world_action(action_id: ActionId, effect: ActionEffect) -> Self {
        Self {
            action_id,
            phase1_implemented: true,
            effect,
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

    pub fn register_phase1_movement_open_close(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("move").unwrap(),
            ActionEffect::Move,
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("open").unwrap(),
            ActionEffect::Open,
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("close").unwrap(),
            ActionEffect::Close,
        ));
    }

    pub fn register_phase1_take_place(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("take").unwrap(),
            ActionEffect::Take,
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("place").unwrap(),
            ActionEffect::Place,
        ));
    }

    pub fn register_phase1_inspect_wait(&mut self) {
        self.register(ActionDefinition::query_only(ActionId::new("look").unwrap()));
        self.register(ActionDefinition::query_only(
            ActionId::new("inspect_place").unwrap(),
        ));
        self.register(ActionDefinition::query_only(
            ActionId::new("inspect_entity").unwrap(),
        ));
        self.register(ActionDefinition::world_action(
            ActionId::new("wait").unwrap(),
            ActionEffect::Wait,
        ));
    }

    pub fn register_phase2a_epistemics(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("check_container").unwrap(),
            ActionEffect::CheckContainer,
        ));
        self.register(ActionDefinition::query_only(
            ActionId::new("truthful_accuse_probe").unwrap(),
        ));
    }

    pub fn register_phase3a_sleep(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("sleep").unwrap(),
            ActionEffect::Sleep,
        ));
    }

    pub fn register_phase3a_eat(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("eat").unwrap(),
            ActionEffect::Eat,
        ));
    }

    pub fn register_phase3a_work(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("work_block").unwrap(),
            ActionEffect::Work,
        ));
    }

    pub fn register_phase3a_continue_routine(&mut self) {
        self.register(ActionDefinition::world_action(
            ActionId::new("continue_routine").unwrap(),
            ActionEffect::ContinueRoutine,
        ));
    }
}
