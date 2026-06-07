use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::agent::{BlockerCategory, HiddenTruthAudit, RoutineStep};
use crate::epistemics::EpistemicProjection;
use crate::ids::{ActionId, ActorId, ContainerId, PlaceId};
use crate::state::AgentState;

pub const DEFAULT_PLANNER_BUDGET: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorKnownFact {
    pub stable_id: String,
    pub proof_source: ActorKnownFactProofSource,
}

impl ActorKnownFact {
    pub fn modeled(stable_id: impl Into<String>, proof_source: impl Into<String>) -> Self {
        Self {
            stable_id: stable_id.into(),
            proof_source: ActorKnownFactProofSource::Modeled(proof_source.into()),
        }
    }

    pub fn unproven(stable_id: impl Into<String>, note: impl Into<String>) -> Self {
        Self {
            stable_id: stable_id.into(),
            proof_source: ActorKnownFactProofSource::NoModeledSource(note.into()),
        }
    }

    pub fn is_actor_known(&self) -> bool {
        matches!(self.proof_source, ActorKnownFactProofSource::Modeled(_))
    }

    pub fn proof_note(&self) -> String {
        match &self.proof_source {
            ActorKnownFactProofSource::Modeled(source) => {
                format!("{}={source}", self.stable_id)
            }
            ActorKnownFactProofSource::NoModeledSource(note) => {
                format!("{}=unproven:{note}", self.stable_id)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActorKnownFactProofSource {
    Modeled(String),
    NoModeledSource(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlannerGoal {
    ReachPlace(PlaceId),
    CheckContainer(ContainerId),
    EatKnownFood(String),
    WaitWithReason(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorKnownPlanningState {
    pub actor_id: ActorId,
    pub current_place_id: PlaceId,
    pub known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    pub known_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
    pub known_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
    pub known_food_sources: BTreeSet<String>,
    pub proof_sources: Vec<String>,
    pub actor_known_facts: Vec<ActorKnownFact>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisibleLocalPlanningState {
    pub current_place_id: PlaceId,
    pub visible_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    pub visible_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
    pub visible_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
    pub visible_food_sources: BTreeSet<String>,
}

pub fn build_actor_known_planning_state(
    actor_id: &ActorId,
    epistemic_projection: &EpistemicProjection,
    agent_state: &AgentState,
    visible_local: &VisibleLocalPlanningState,
) -> ActorKnownPlanningState {
    let mut actor_known_facts = vec![ActorKnownFact::modeled(
        "actor_current_place_visible",
        format!(
            "visible_local:current_place:{}",
            visible_local.current_place_id.as_str()
        ),
    )];
    if agent_state.needs_by_actor.contains_key(actor_id) {
        actor_known_facts.push(ActorKnownFact::modeled(
            "agent_needs_present",
            "agent_state:needs_present",
        ));
    }
    let actor_belief_count = epistemic_projection
        .beliefs_by_holder
        .get(actor_id)
        .map_or(0, BTreeSet::len);
    actor_known_facts.push(ActorKnownFact::modeled(
        "actor_belief_projection",
        format!("epistemic_projection:actor_beliefs:{actor_belief_count}"),
    ));
    for (from, tos) in &visible_local.visible_edges {
        for to in tos {
            actor_known_facts.push(ActorKnownFact::modeled(
                "known_route_surface",
                format!("visible_local:edge:{}->{}", from.as_str(), to.as_str()),
            ));
        }
    }
    for food_source in &visible_local.visible_food_sources {
        actor_known_facts.push(ActorKnownFact::modeled(
            "actor_knows_food_source",
            format!("visible_local:food:{food_source}"),
        ));
    }
    actor_known_facts.sort_by(|left, right| {
        left.stable_id
            .cmp(&right.stable_id)
            .then_with(|| left.proof_note().cmp(&right.proof_note()))
    });
    let proof_sources = actor_known_facts
        .iter()
        .map(ActorKnownFact::proof_note)
        .collect::<Vec<_>>();

    ActorKnownPlanningState {
        actor_id: actor_id.clone(),
        current_place_id: visible_local.current_place_id.clone(),
        known_edges: visible_local.visible_edges.clone(),
        known_closed_doors: visible_local.visible_closed_doors.clone(),
        known_containers_by_place: visible_local.visible_containers_by_place.clone(),
        known_food_sources: visible_local.visible_food_sources.clone(),
        proof_sources,
        actor_known_facts,
    }
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
    state: &ActorKnownPlanningState,
    request: &LocalPlanRequest,
) -> Result<LocalPlan, LocalPlanFailure> {
    match &request.goal {
        PlannerGoal::ReachPlace(target) => plan_route(state, request, target),
        PlannerGoal::CheckContainer(container_id) => {
            plan_check_container(state, request, container_id)
        }
        PlannerGoal::EatKnownFood(food_source) => plan_known_food(state, request, food_source),
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
fn plan_route(
    state: &ActorKnownPlanningState,
    request: &LocalPlanRequest,
    target: &PlaceId,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state.current_place_id == *target {
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

    let mut queue = VecDeque::from([(state.current_place_id.clone(), Vec::<PlaceId>::new())]);
    let mut visited = BTreeSet::from([state.current_place_id.clone()]);
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
            .known_edges
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
                let proposals = proposals_for_path(state, &state.current_place_id, &next_path);
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
    state: &ActorKnownPlanningState,
    start: &PlaceId,
    path: &[PlaceId],
) -> Vec<PlannedProposal> {
    let mut proposals = Vec::new();
    let mut from = start.clone();
    for to in path {
        if let Some(door_id) = state.known_closed_doors.get(&(from.clone(), to.clone())) {
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
    state: &ActorKnownPlanningState,
    request: &LocalPlanRequest,
    container_id: &ContainerId,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state
        .known_containers_by_place
        .get(&state.current_place_id)
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
    state: &ActorKnownPlanningState,
    request: &LocalPlanRequest,
    food_source: &str,
) -> Result<LocalPlan, LocalPlanFailure> {
    if state.known_food_sources.contains(food_source) {
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
    state: &ActorKnownPlanningState,
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
    state: &ActorKnownPlanningState,
    request: &LocalPlanRequest,
    candidates_tried: Vec<String>,
    selected_plan: Vec<PlannedProposal>,
    rejected_steps: Vec<String>,
    blocker: Option<BlockerCategory>,
) -> LocalPlanTrace {
    LocalPlanTrace {
        actor_id: state.actor_id.clone(),
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

pub fn derive_hidden_truth_audit(
    state: &ActorKnownPlanningState,
    request_facts: &[ActorKnownFact],
) -> HiddenTruthAudit {
    let mut proof_notes = state.proof_sources.clone();
    proof_notes.extend(request_facts.iter().map(ActorKnownFact::proof_note));
    proof_notes.sort();
    proof_notes.dedup();
    let actor_known_only = state
        .actor_known_facts
        .iter()
        .chain(request_facts.iter())
        .all(ActorKnownFact::is_actor_known);
    HiddenTruthAudit {
        actor_known_only,
        notes: format!("planner proof_sources={}", proof_notes.join(",")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::SemanticActionId;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn known_state() -> ActorKnownPlanningState {
        let home = place("home");
        let street = place("street");
        let office = place("office");
        let mut known_edges = BTreeMap::new();
        known_edges.insert(home.clone(), BTreeSet::from([street.clone()]));
        known_edges.insert(street.clone(), BTreeSet::from([office.clone()]));
        ActorKnownPlanningState {
            actor_id: actor_id(),
            current_place_id: home.clone(),
            known_edges,
            known_closed_doors: BTreeMap::from([(
                (home, street.clone()),
                "door_home_street".to_string(),
            )]),
            known_containers_by_place: BTreeMap::new(),
            known_food_sources: BTreeSet::from(["food_soup_pot".to_string()]),
            proof_sources: vec!["test:known_state".to_string()],
            actor_known_facts: vec![ActorKnownFact::modeled(
                "actor_knows_food_source",
                "test:known_state",
            )],
        }
    }

    fn request(goal: PlannerGoal, budget: usize) -> LocalPlanRequest {
        LocalPlanRequest {
            routine_step: RoutineStep::MoveTowardPlace {
                action_id: SemanticActionId::new("move").unwrap(),
            },
            goal,
            budget,
            actor_known_facts: vec![
                ActorKnownFact::modeled("known_place", "test:home"),
                ActorKnownFact::modeled("known_place", "test:office"),
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
                actor_known_facts: vec![ActorKnownFact::modeled(
                    "modeled_wait_reason",
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
            .contains("modeled_wait_reason=test:visible_schedule"));

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
