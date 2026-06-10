use std::collections::{BTreeSet, VecDeque};

pub use crate::agent::actor_known::{
    build_actor_known_planning_state, build_actor_known_planning_state_with_projection_limitation,
    derive_hidden_truth_audit, ActorKnownFact,
    ActorKnownPlanningContext as ActorKnownPlanningState, ActorKnownProvenance,
    VisibleLocalPlanningState,
};
use crate::agent::{ActorKnownPlanningContext, BlockerCategory, HiddenTruthAudit, RoutineStep};
use crate::ids::{ActionId, ActorId, ContainerId, PlaceId};

pub const DEFAULT_PLANNER_BUDGET: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlannerGoal {
    ReachPlace(PlaceId),
    CheckContainer(ContainerId),
    EatKnownFood(String),
    StartSleep(PlaceId),
    StartWorkBlock(String),
    WaitWithReason(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalPlanRequest {
    pub routine_step: RoutineStep,
    pub goal: PlannerGoal,
    pub budget: usize,
    pub actor_known_facts: Vec<ActorKnownFact>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedProposal {
    pub action_id: ActionId,
    pub target_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalPlanTrace {
    pub actor_id: ActorId,
    pub inputs: Vec<String>,
    pub candidates_tried: Vec<String>,
    pub selected_plan: Vec<PlannedProposal>,
    pub rejected_steps: Vec<String>,
    pub blocker: Option<BlockerCategory>,
    pub hidden_truth_audit_result: HiddenTruthAudit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalPlan {
    pub proposals: Vec<PlannedProposal>,
    pub trace: LocalPlanTrace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalPlanFailure {
    pub blocker: BlockerCategory,
    pub reason: String,
    pub trace: LocalPlanTrace,
}

#[allow(clippy::result_large_err)]
pub fn plan_local_actions(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
) -> Result<LocalPlan, LocalPlanFailure> {
    match &request.goal {
        PlannerGoal::ReachPlace(target) => plan_route(state, request, target),
        PlannerGoal::CheckContainer(container_id) => {
            plan_check_container(state, request, container_id)
        }
        PlannerGoal::EatKnownFood(food_source) => plan_known_food(state, request, food_source),
        PlannerGoal::StartSleep(sleep_place_id) => plan_start_sleep(state, request, sleep_place_id),
        PlannerGoal::StartWorkBlock(workplace_id) => {
            plan_start_work_block(state, request, workplace_id)
        }
        PlannerGoal::WaitWithReason(reason) => {
            let proposal = PlannedProposal {
                action_id: ActionId::new("wait").unwrap(),
                target_ids: vec![reason.clone()],
            };
            Ok(LocalPlan {
                proposals: vec![proposal.clone()],
                trace: trace(
                    state,
                    request,
                    vec![format!("wait:{reason}")],
                    vec![proposal],
                    Vec::new(),
                    None,
                ),
            })
        }
    }
}

#[allow(clippy::result_large_err)]
fn plan_start_sleep(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
    sleep_place_id: &PlaceId,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state.current_place_id() == sleep_place_id
        && state.known_sleep_places().contains(sleep_place_id)
    {
        let proposal = PlannedProposal {
            action_id: ActionId::new("sleep").unwrap(),
            target_ids: vec![sleep_place_id.as_str().to_string()],
        };
        return Ok(LocalPlan {
            proposals: vec![proposal.clone()],
            trace: trace(
                state,
                request,
                vec![format!("sleep_place:{}", sleep_place_id.as_str())],
                vec![proposal],
                Vec::new(),
                None,
            ),
        });
    }

    Err(failure(
        state,
        request,
        BlockerCategory::Knowledge,
        "sleep place is not actor-known at current place",
        vec![format!("sleep_place:{}", sleep_place_id.as_str())],
    ))
}

#[allow(clippy::result_large_err)]
fn plan_start_work_block(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
    workplace_id: &str,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state
        .known_workplaces()
        .iter()
        .any(|(known_workplace_id, place_id)| {
            known_workplace_id.as_str() == workplace_id && place_id == state.current_place_id()
        })
    {
        let proposal = PlannedProposal {
            action_id: ActionId::new("work_block").unwrap(),
            target_ids: vec![workplace_id.to_string()],
        };
        return Ok(LocalPlan {
            proposals: vec![proposal.clone()],
            trace: trace(
                state,
                request,
                vec![format!("workplace:{workplace_id}")],
                vec![proposal],
                Vec::new(),
                None,
            ),
        });
    }

    Err(failure(
        state,
        request,
        BlockerCategory::Knowledge,
        "workplace is not actor-known at current place",
        vec![format!("workplace:{workplace_id}")],
    ))
}

#[allow(clippy::result_large_err)]
fn plan_route(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
    target: &PlaceId,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state.current_place_id() == target {
        return Ok(LocalPlan {
            proposals: Vec::new(),
            trace: trace(
                state,
                request,
                vec!["already_at_target".to_string()],
                Vec::new(),
                Vec::new(),
                None,
            ),
        });
    }

    let mut queue = VecDeque::from([(state.current_place_id().clone(), Vec::<PlaceId>::new())]);
    let mut visited = BTreeSet::from([state.current_place_id().clone()]);
    let mut candidates = Vec::new();
    let mut expansions = 0_usize;

    while let Some((place_id, path)) = queue.pop_front() {
        if expansions >= request.budget {
            return Err(failure(
                state,
                request,
                BlockerCategory::PlannerBudgetExhausted,
                "planner budget exhausted",
                candidates,
            ));
        }
        expansions += 1;
        let neighbors = state
            .known_edges()
            .get(&place_id)
            .cloned()
            .unwrap_or_default();
        for neighbor in neighbors {
            candidates.push(format!("edge:{}->{}", place_id.as_str(), neighbor.as_str()));
            if !visited.insert(neighbor.clone()) {
                continue;
            }
            let mut next_path = path.clone();
            next_path.push(neighbor.clone());
            if &neighbor == target {
                let proposals = proposals_for_path(state, state.current_place_id(), &next_path);
                return Ok(LocalPlan {
                    trace: trace(
                        state,
                        request,
                        candidates,
                        proposals.clone(),
                        Vec::new(),
                        None,
                    ),
                    proposals,
                });
            }
            queue.push_back((neighbor, next_path));
        }
    }

    Err(failure(
        state,
        request,
        BlockerCategory::Knowledge,
        "no actor-known route to target",
        candidates,
    ))
}

fn proposals_for_path(
    state: &ActorKnownPlanningContext,
    start: &PlaceId,
    path: &[PlaceId],
) -> Vec<PlannedProposal> {
    let mut proposals = Vec::new();
    let mut from = start.clone();
    for to in path {
        if let Some(door_id) = state.known_closed_doors().get(&(from.clone(), to.clone())) {
            proposals.push(PlannedProposal {
                action_id: ActionId::new("open").unwrap(),
                target_ids: vec![door_id.clone()],
            });
        }
        proposals.push(PlannedProposal {
            action_id: ActionId::new("move").unwrap(),
            target_ids: vec![to.to_string()],
        });
        from = to.clone();
    }
    proposals
}

#[allow(clippy::result_large_err)]
fn plan_check_container(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
    container_id: &ContainerId,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state
        .known_containers_by_place()
        .get(state.current_place_id())
        .is_some_and(|containers| containers.contains(container_id))
    {
        let proposal = PlannedProposal {
            action_id: ActionId::new("check_container").unwrap(),
            target_ids: vec![container_id.to_string()],
        };
        return Ok(LocalPlan {
            proposals: vec![proposal.clone()],
            trace: trace(
                state,
                request,
                vec![format!("container:{}", container_id.as_str())],
                vec![proposal],
                Vec::new(),
                None,
            ),
        });
    }

    Err(failure(
        state,
        request,
        BlockerCategory::Knowledge,
        "container is not actor-known at current place",
        vec![format!("container:{}", container_id.as_str())],
    ))
}

#[allow(clippy::result_large_err)]
fn plan_known_food(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
    food_source: &str,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state.known_food_sources().contains(food_source) {
        let proposal = PlannedProposal {
            action_id: ActionId::new("eat").unwrap(),
            target_ids: vec![food_source.to_string()],
        };
        return Ok(LocalPlan {
            proposals: vec![proposal.clone()],
            trace: trace(
                state,
                request,
                vec![format!("food:{food_source}")],
                vec![proposal],
                Vec::new(),
                None,
            ),
        });
    }

    Err(failure(
        state,
        request,
        BlockerCategory::Resource,
        "food source is not actor-known",
        vec![format!("food:{food_source}")],
    ))
}

fn failure(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
    blocker: BlockerCategory,
    reason: &str,
    candidates: Vec<String>,
) -> LocalPlanFailure {
    LocalPlanFailure {
        blocker,
        reason: reason.to_string(),
        trace: trace(
            state,
            request,
            candidates,
            Vec::new(),
            vec![reason.to_string()],
            Some(blocker),
        ),
    }
}

fn trace(
    state: &ActorKnownPlanningContext,
    request: &LocalPlanRequest,
    candidates_tried: Vec<String>,
    selected_plan: Vec<PlannedProposal>,
    rejected_steps: Vec<String>,
    blocker: Option<BlockerCategory>,
) -> LocalPlanTrace {
    LocalPlanTrace {
        actor_id: state.actor_id().clone(),
        inputs: request
            .actor_known_facts
            .iter()
            .map(ActorKnownFact::proof_note)
            .collect(),
        candidates_tried,
        selected_plan,
        rejected_steps,
        blocker,
        hidden_truth_audit_result: derive_hidden_truth_audit(state, &request.actor_known_facts),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::SemanticActionId;
    use std::collections::BTreeMap;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn observed_fact(stable_id: &str, value: &str, source: &str) -> ActorKnownFact {
        ActorKnownFact::observed_now(actor_id(), stable_id, value, source, None, test_source())
    }

    fn remembered_fact(stable_id: &str, value: &str, source: &str) -> ActorKnownFact {
        ActorKnownFact::remembered_belief(actor_id(), stable_id, value, source, None, test_source())
    }

    fn test_source() -> crate::agent::SourceEventIds {
        crate::agent::SourceEventIds::checked(vec![crate::ids::EventId::new(
            "event_test_actor_known",
        )
        .unwrap()])
        .unwrap()
    }

    fn known_state() -> ActorKnownPlanningState {
        let home = place("home");
        let street = place("street");
        let office = place("office");
        let mut known_edges = BTreeMap::new();
        known_edges.insert(home.clone(), BTreeSet::from([street.clone()]));
        known_edges.insert(street.clone(), BTreeSet::from([office.clone()]));
        ActorKnownPlanningState::from_observed_parts(
            actor_id(),
            home.clone(),
            known_edges,
            BTreeMap::from([((home, street.clone()), "door_home_street".to_string())]),
            BTreeMap::new(),
            BTreeSet::from(["food_soup_pot".to_string()]),
            BTreeSet::from([place("home")]),
            BTreeMap::new(),
            vec![observed_fact(
                "actor_knows_food_source",
                "food_soup_pot",
                "test:known_state",
            )],
        )
    }

    fn request(goal: PlannerGoal, budget: usize) -> LocalPlanRequest {
        LocalPlanRequest {
            routine_step: RoutineStep::MoveTowardPlace {
                action_id: SemanticActionId::new("move").unwrap(),
            },
            goal,
            budget,
            actor_known_facts: vec![
                remembered_fact("known_place", "home", "test:home"),
                remembered_fact("known_place", "office", "test:office"),
            ],
        }
    }

    #[test]
    fn planner_sequences_open_then_move_to_known_place() {
        let plan = plan_local_actions(
            &known_state(),
            &request(
                PlannerGoal::ReachPlace(place("office")),
                DEFAULT_PLANNER_BUDGET,
            ),
        )
        .unwrap();

        let actions = plan
            .proposals
            .iter()
            .map(|proposal| proposal.action_id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(actions, ["open", "move", "move"]);
        assert!(plan.trace.hidden_truth_audit_result.actor_known_only);
    }

    #[test]
    fn budget_exhaustion_reports_candidates_tried() {
        let failure = plan_local_actions(
            &known_state(),
            &request(PlannerGoal::ReachPlace(place("office")), 1),
        )
        .unwrap_err();

        assert_eq!(failure.blocker, BlockerCategory::PlannerBudgetExhausted);
        assert!(!failure.trace.candidates_tried.is_empty());
        assert_eq!(
            failure.trace.blocker,
            Some(BlockerCategory::PlannerBudgetExhausted)
        );
    }

    #[test]
    fn believed_but_wrong_food_source_fails_without_truth_lookup() {
        let failure = plan_local_actions(
            &known_state(),
            &request(
                PlannerGoal::EatKnownFood("hidden_food_pantry".to_string()),
                4,
            ),
        )
        .unwrap_err();

        assert_eq!(failure.blocker, BlockerCategory::Resource);
        assert!(failure.trace.hidden_truth_audit_result.actor_known_only);
        assert!(!failure
            .trace
            .inputs
            .iter()
            .any(|input| input.contains("hidden_food_pantry")));
    }

    #[test]
    fn identical_known_state_yields_identical_plan() {
        let first = plan_local_actions(
            &known_state(),
            &request(
                PlannerGoal::ReachPlace(place("office")),
                DEFAULT_PLANNER_BUDGET,
            ),
        )
        .unwrap();
        let second = plan_local_actions(
            &known_state(),
            &request(
                PlannerGoal::ReachPlace(place("office")),
                DEFAULT_PLANNER_BUDGET,
            ),
        )
        .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn hidden_truth_audit_is_derived_from_fact_provenance() {
        let known = plan_local_actions(
            &known_state(),
            &LocalPlanRequest {
                routine_step: RoutineStep::WaitUntil {
                    reason: "test".to_string(),
                },
                goal: PlannerGoal::WaitWithReason("test".to_string()),
                budget: 1,
                actor_known_facts: vec![remembered_fact(
                    "modeled_wait_reason",
                    "test",
                    "test:visible_schedule",
                )],
            },
        )
        .unwrap();
        assert!(known.trace.hidden_truth_audit_result.actor_known_only);
        assert!(known
            .trace
            .hidden_truth_audit_result
            .notes
            .contains("modeled_wait_reason=remembered_belief:test:visible_schedule"));

        let unproven = plan_local_actions(
            &known_state(),
            &LocalPlanRequest {
                routine_step: RoutineStep::WaitUntil {
                    reason: "test".to_string(),
                },
                goal: PlannerGoal::WaitWithReason("test".to_string()),
                budget: 1,
                actor_known_facts: vec![ActorKnownFact::unproven(
                    "modeled_wait_reason",
                    "caller supplied no modeled source",
                )],
            },
        )
        .unwrap();
        assert!(!unproven.trace.hidden_truth_audit_result.actor_known_only);
        assert!(unproven
            .trace
            .hidden_truth_audit_result
            .notes
            .contains("unproven:caller supplied no modeled source"));
    }
}
