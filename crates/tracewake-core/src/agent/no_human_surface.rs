use std::collections::{BTreeMap, BTreeSet};

use crate::agent::{
    build_actor_known_planning_state_with_projection_limitation, ActorKnownFact,
    ActorKnownPlanningContext, RoutineFamily, VisibleLocalPlanningState,
};
use crate::ids::{ActorId, PlaceId, WorkplaceId};
use crate::location::Location;
use crate::state::{AgentState, PhysicalState};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SealedActorKnownSurface {
    context: ActorKnownPlanningContext,
}

impl SealedActorKnownSurface {
    fn new(context: ActorKnownPlanningContext) -> Self {
        Self { context }
    }

    pub fn context(&self) -> &ActorKnownPlanningContext {
        &self.context
    }

    pub fn into_context(self) -> ActorKnownPlanningContext {
        self.context
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoHumanActorKnownSurfaceBuilder {
    actor_id: ActorId,
    current_place_id: PlaceId,
    decision_tick: Option<SimTick>,
    known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    known_food_sources: BTreeSet<String>,
    known_sleep_places: BTreeSet<PlaceId>,
    known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
    facts: Vec<ActorKnownFact>,
}

impl NoHumanActorKnownSurfaceBuilder {
    pub fn new(
        actor_id: ActorId,
        current_place_id: PlaceId,
        decision_tick: Option<SimTick>,
    ) -> Self {
        Self {
            actor_id,
            current_place_id,
            decision_tick,
            known_edges: BTreeMap::new(),
            known_food_sources: BTreeSet::new(),
            known_sleep_places: BTreeSet::new(),
            known_workplaces: BTreeMap::new(),
            facts: Vec::new(),
        }
    }

    pub fn from_modeled_observations(
        state: &PhysicalState,
        agent_state: &AgentState,
        actor_id: ActorId,
        current_place_id: PlaceId,
        decision_tick: Option<SimTick>,
    ) -> Self {
        let mut builder = Self::new(actor_id, current_place_id, decision_tick);
        builder.observe_visible_routes_from_current_place(state);
        builder.observe_visible_food_sources_from_current_place(state);
        builder.observe_sleep_notice_from_active_routine(state, agent_state);
        builder.observe_workplace_notices_from_active_routines(state, agent_state);
        builder
    }

    pub fn with_role_assignment_notice(
        mut self,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        source: impl Into<String>,
    ) -> Self {
        self.add_role_assignment_notice(workplace_id, place_id, source);
        self
    }

    pub fn with_sleep_place_knowledge(
        mut self,
        sleep_place_id: PlaceId,
        source: impl Into<String>,
    ) -> Self {
        let source = source.into();
        self.known_sleep_places.insert(sleep_place_id.clone());
        self.facts.push(ActorKnownFact::remembered_belief(
            self.actor_id.clone(),
            "actor_knows_sleep_place",
            sleep_place_id.as_str(),
            format!("sleep_place_knowledge:{source}"),
            self.decision_tick,
        ));
        if sleep_place_id == self.current_place_id {
            self.facts.push(ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "sleep_place_believed_accessible",
                sleep_place_id.as_str(),
                "modeled_observation:sleep_place_accessible",
                self.decision_tick,
            ));
        }
        self
    }

    pub fn build(self, agent_state: &AgentState) -> SealedActorKnownSurface {
        let visible_local = VisibleLocalPlanningState::new(
            self.current_place_id,
            self.known_edges,
            BTreeMap::new(),
            BTreeMap::new(),
            self.known_food_sources,
            self.known_sleep_places,
            self.known_workplaces,
        );
        let mut context = build_actor_known_planning_state_with_projection_limitation(
            &self.actor_id,
            agent_state,
            &visible_local,
        );
        context.extend_actor_known_facts(self.facts);
        SealedActorKnownSurface::new(context)
    }

    fn observe_visible_routes_from_current_place(&mut self, state: &PhysicalState) {
        if let Some(current_place) = state.places().get(&self.current_place_id) {
            self.known_edges.insert(
                self.current_place_id.clone(),
                current_place.adjacent_place_ids.clone(),
            );
        }
    }

    fn observe_visible_food_sources_from_current_place(&mut self, state: &PhysicalState) {
        self.known_food_sources = state
            .food_supplies()
            .values()
            .filter(|food| {
                matches!(&food.location, Location::AtPlace(place_id) if place_id == &self.current_place_id)
            })
            .map(|food| food.food_supply_id.as_str().to_string())
            .collect();
    }

    fn observe_sleep_notice_from_active_routine(
        &mut self,
        state: &PhysicalState,
        agent_state: &AgentState,
    ) {
        if !has_live_routine_family(
            agent_state,
            &self.actor_id,
            self.decision_tick,
            RoutineFamily::SleepNight,
        ) {
            return;
        }
        let Some((sleep_affordance_id, _)) =
            state
                .sleep_affordances()
                .iter()
                .find(|(_, sleep_affordance)| {
                    sleep_affordance.access_open
                        && sleep_affordance.place_id == self.current_place_id
                })
        else {
            return;
        };
        let current_place_id = self.current_place_id.clone();
        self.known_sleep_places.insert(current_place_id.clone());
        self.facts.push(ActorKnownFact::observed_now(
            self.actor_id.clone(),
            "actor_knows_sleep_affordance",
            sleep_affordance_id.as_str(),
            "modeled_observation:open_sleep_affordance_at_current_place",
            self.decision_tick,
        ));
        self.facts.push(ActorKnownFact::remembered_belief(
            self.actor_id.clone(),
            "actor_knows_sleep_place",
            current_place_id.as_str(),
            "routine_assignment_notice:sleep_window_current_rest_surface",
            self.decision_tick,
        ));
        self.facts.push(ActorKnownFact::observed_now(
            self.actor_id.clone(),
            "sleep_place_believed_accessible",
            current_place_id.as_str(),
            "routine_assignment_notice:sleep_window_current_rest_surface",
            self.decision_tick,
        ));
    }

    fn observe_workplace_notices_from_active_routines(
        &mut self,
        state: &PhysicalState,
        agent_state: &AgentState,
    ) {
        let has_work_routine = [RoutineFamily::GoToWork, RoutineFamily::WorkBlock]
            .into_iter()
            .any(|family| {
                has_live_routine_family(agent_state, &self.actor_id, self.decision_tick, family)
            });
        if !has_work_routine {
            return;
        }

        for (workplace_id, workplace) in state.workplaces() {
            if workplace.assigned_actor_ids.contains(&self.actor_id) {
                self.add_role_assignment_notice(
                    workplace_id.clone(),
                    workplace.place_id.clone(),
                    "routine_assignment_workplace_notice",
                );
            }
        }
    }

    fn add_role_assignment_notice(
        &mut self,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        source: impl Into<String>,
    ) {
        let source = source.into();
        self.known_workplaces
            .insert(workplace_id.clone(), place_id.clone());
        self.facts.push(ActorKnownFact::routine_assignment(
            self.actor_id.clone(),
            "actor_knows_workplace",
            format!("{}@{}", workplace_id.as_str(), place_id.as_str()),
            format!("role_assignment_notice:{source}"),
            self.decision_tick,
        ));
        self.facts.push(ActorKnownFact::routine_assignment(
            self.actor_id.clone(),
            "workplace_assignment_active",
            workplace_id.as_str(),
            format!("role_assignment_notice:{source}"),
            self.decision_tick,
        ));
        if place_id == self.current_place_id {
            self.facts.push(ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "actor_at_workplace",
                place_id.as_str(),
                "modeled_observation:actor_at_noticed_workplace",
                self.decision_tick,
            ));
            self.facts.push(ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "assigned_workplace_known",
                place_id.as_str(),
                "modeled_observation:assigned_workplace_known",
                self.decision_tick,
            ));
            self.facts.push(ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "at_workplace",
                place_id.as_str(),
                "modeled_observation:at_workplace",
                self.decision_tick,
            ));
        }
    }
}

fn has_live_routine_family(
    agent_state: &AgentState,
    actor_id: &ActorId,
    decision_tick: Option<SimTick>,
    family: RoutineFamily,
) -> bool {
    agent_state.routine_executions().values().any(|execution| {
        &execution.actor_id == actor_id
            && execution.family == family
            && decision_tick.is_none_or(|tick| {
                execution.start_tick <= tick
                    && execution
                        .deadline_tick
                        .is_none_or(|deadline| tick < deadline)
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ActorBody, FoodSupplyState, PlaceState, WorkplaceState};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn workplace_id() -> WorkplaceId {
        WorkplaceId::new("workplace_tomas").unwrap()
    }

    #[test]
    fn raw_workplace_assignment_is_not_actor_known_without_notice() {
        let actor_id = actor_id();
        let home = place_id("home_tomas");
        let workshop = place_id("workshop_tomas");
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), home.clone()),
        );
        state
            .places
            .insert(home.clone(), PlaceState::new(home.clone(), "Home"));
        state.places.insert(
            workshop.clone(),
            PlaceState::new(workshop.clone(), "Workshop"),
        );
        let mut workplace = WorkplaceState::new(workplace_id(), workshop, "work_done");
        workplace.assigned_actor_ids.insert(actor_id.clone());
        state
            .workplaces
            .insert(workplace.workplace_id.clone(), workplace);

        let surface = NoHumanActorKnownSurfaceBuilder::from_modeled_observations(
            &state,
            &AgentState::default(),
            actor_id,
            home,
            Some(SimTick::ZERO),
        )
        .build(&AgentState::default());

        assert!(surface.context().known_workplaces().is_empty());
        assert!(!surface
            .context()
            .actor_known_facts()
            .iter()
            .any(|fact| fact.stable_id() == "actor_knows_workplace"));
    }

    #[test]
    fn current_place_is_not_implicit_sleep_knowledge() {
        let actor_id = actor_id();
        let home = place_id("home_tomas");
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), home.clone()),
        );
        state
            .places
            .insert(home.clone(), PlaceState::new(home.clone(), "Home"));

        let surface = NoHumanActorKnownSurfaceBuilder::from_modeled_observations(
            &state,
            &AgentState::default(),
            actor_id,
            home,
            Some(SimTick::ZERO),
        )
        .build(&AgentState::default());

        assert!(surface.context().known_sleep_places().is_empty());
        assert!(!surface
            .context()
            .actor_known_facts()
            .iter()
            .any(|fact| fact.stable_id() == "sleep_place_believed_accessible"));
    }

    #[test]
    fn workplace_becomes_known_only_from_modeled_notice() {
        let actor_id = actor_id();
        let workshop = place_id("workshop_tomas");
        let state = PhysicalState::default();

        let surface = NoHumanActorKnownSurfaceBuilder::from_modeled_observations(
            &state,
            &AgentState::default(),
            actor_id,
            workshop.clone(),
            Some(SimTick::ZERO),
        )
        .with_role_assignment_notice(workplace_id(), workshop, "fixture_assignment_notice")
        .build(&AgentState::default());

        assert_eq!(surface.context().known_workplaces().len(), 1);
        assert!(surface
            .context()
            .actor_known_facts()
            .iter()
            .any(|fact| fact.proof_note().contains("role_assignment_notice")));
    }

    #[test]
    fn visible_food_observation_still_requires_local_visibility() {
        let actor_id = actor_id();
        let kitchen = place_id("kitchen");
        let cellar = place_id("cellar");
        let mut state = PhysicalState::default();
        state
            .places
            .insert(kitchen.clone(), PlaceState::new(kitchen.clone(), "Kitchen"));
        state
            .food_supplies
            .insert(crate::ids::FoodSupplyId::new("visible_meal").unwrap(), {
                FoodSupplyState::new(
                    crate::ids::FoodSupplyId::new("visible_meal").unwrap(),
                    Location::AtPlace(kitchen.clone()),
                    1,
                    100,
                )
            });
        state
            .food_supplies
            .insert(crate::ids::FoodSupplyId::new("hidden_meal").unwrap(), {
                FoodSupplyState::new(
                    crate::ids::FoodSupplyId::new("hidden_meal").unwrap(),
                    Location::AtPlace(cellar),
                    1,
                    100,
                )
            });

        let surface = NoHumanActorKnownSurfaceBuilder::from_modeled_observations(
            &state,
            &AgentState::default(),
            actor_id,
            kitchen,
            Some(SimTick::ZERO),
        )
        .build(&AgentState::default());

        assert!(surface
            .context()
            .known_food_sources()
            .contains("visible_meal"));
        assert!(!surface
            .context()
            .known_food_sources()
            .contains("hidden_meal"));
    }
}
