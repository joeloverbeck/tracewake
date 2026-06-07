use crate::agent::{GoalKind, RoutineFamily, RoutineStep, RoutineStepProposal, RoutineTemplate};
use crate::ids::{RoutineTemplateId, SemanticActionId};

pub fn phase3a_routine_templates() -> Vec<RoutineTemplate> {
    vec![
        template(
            "routine_morning_wake",
            RoutineFamily::MorningWake,
            vec!["actor_is_resting"],
            vec!["sleep_state_can_end"],
            vec![
                step("continue_current_step", "continue_routine"),
                wait("morning reevaluation"),
            ],
            vec!["sleep_end_blocked"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_eat_meal",
            RoutineFamily::EatMeal,
            vec!["actor_knows_food_source"],
            vec!["food_source_believed_accessible"],
            vec![
                step("check_known_container", "check_container"),
                step("consume_accessible_food", "eat"),
            ],
            vec!["food_missing", "food_inaccessible"],
            vec!["fallback_find_food", "wait_with_reason"],
            Some("food_source"),
        ),
        template(
            "routine_go_to_work",
            RoutineFamily::GoToWork,
            vec!["actor_knows_workplace"],
            vec!["route_planner_available"],
            vec![step("move_toward_place", "move")],
            vec!["route_blocked", "workplace_unknown"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_work_block",
            RoutineFamily::WorkBlock,
            vec!["workplace_assignment_active"],
            vec!["actor_at_workplace"],
            vec![step("start_work_block", "work_block")],
            vec!["workplace_closed", "need_blocked"],
            vec!["fallback_wait_with_reason"],
            Some("workstation"),
        ),
        template(
            "routine_return_home",
            RoutineFamily::ReturnHome,
            vec!["actor_knows_home"],
            vec!["route_planner_available"],
            vec![step("move_toward_place", "move")],
            vec!["route_home_blocked"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_sleep_night",
            RoutineFamily::SleepNight,
            vec!["actor_knows_sleep_place"],
            vec!["sleep_place_believed_accessible"],
            vec![
                step("move_toward_place", "move"),
                step("start_scheduled_sleep", "sleep"),
            ],
            vec!["sleep_place_blocked", "sleep_interrupted"],
            vec!["fallback_wait_with_reason"],
            Some("sleep_place"),
        ),
        template(
            "routine_find_food",
            RoutineFamily::FindFood,
            vec!["actor_has_food_search_knowledge"],
            vec!["search_surface_actor_known"],
            vec![
                step("check_known_container", "check_container"),
                step("fallback_to_find_food", "eat"),
            ],
            vec!["no_known_food_sources", "search_blocked"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_continue_current_intention",
            RoutineFamily::ContinueCurrentIntention,
            vec!["active_intention_present"],
            vec!["next_step_available"],
            vec![step("continue_current_step", "continue_routine")],
            vec!["no_current_intention", "step_blocked"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_wait_idle",
            RoutineFamily::Wait,
            vec!["reason_available"],
            vec!["reevaluation_scheduled"],
            vec![wait("bounded idle with reevaluation")],
            vec!["wait_interrupted"],
            vec!["fallback_stuck_diagnostic"],
            None,
        ),
    ]
}

pub fn family_for_goal(goal_kind: GoalKind) -> RoutineFamily {
    match goal_kind {
        GoalKind::Eat => RoutineFamily::EatMeal,
        GoalKind::FindFood => RoutineFamily::FindFood,
        GoalKind::SleepOrRest => RoutineFamily::SleepNight,
        GoalKind::GoToWork => RoutineFamily::GoToWork,
        GoalKind::PerformWorkBlock => RoutineFamily::WorkBlock,
        GoalKind::ReturnHome => RoutineFamily::ReturnHome,
        GoalKind::ContinueCurrentIntention => RoutineFamily::ContinueCurrentIntention,
        GoalKind::IdleWithReason => RoutineFamily::Wait,
        GoalKind::LeaveUnsafePlace => RoutineFamily::Wait,
    }
}

pub fn all_steps_are_proposals(template: &RoutineTemplate) -> bool {
    template.steps.iter().all(|step| match step.proposed() {
        RoutineStepProposal::Action(action_id) => !action_id.as_str().is_empty(),
        RoutineStepProposal::Wait(reason) => !reason.is_empty(),
        RoutineStepProposal::Diagnostic(diagnostic) => !diagnostic.is_empty(),
    })
}

fn step(kind: &str, action_id: &str) -> RoutineStep {
    let action_id = SemanticActionId::new(action_id).unwrap();
    match kind {
        "move_toward_place" => RoutineStep::MoveTowardPlace { action_id },
        "check_known_container" => RoutineStep::CheckKnownContainer { action_id },
        "consume_accessible_food" => RoutineStep::ConsumeAccessibleFood { action_id },
        "start_scheduled_sleep" => RoutineStep::StartScheduledSleep { action_id },
        "start_work_block" => RoutineStep::StartWorkBlock { action_id },
        "continue_current_step" => RoutineStep::ContinueCurrentStep { action_id },
        "fallback_to_find_food" => RoutineStep::FallbackToFindFood { action_id },
        _ => panic!("unsupported routine step kind {kind}"),
    }
}

fn wait(reason: &str) -> RoutineStep {
    RoutineStep::WaitUntil {
        reason: reason.to_string(),
    }
}

fn template(
    id: &str,
    family: RoutineFamily,
    applicability_conditions: Vec<&str>,
    preconditions: Vec<&str>,
    steps: Vec<RoutineStep>,
    failure_modes: Vec<&str>,
    fallback_rules: Vec<&str>,
    reservable_resource: Option<&str>,
) -> RoutineTemplate {
    RoutineTemplate::new(
        RoutineTemplateId::new(id).unwrap(),
        family,
        applicability_conditions
            .into_iter()
            .map(str::to_string)
            .collect(),
        preconditions.into_iter().map(str::to_string).collect(),
        steps,
        1,
        8,
        vec![0],
        failure_modes.into_iter().map(str::to_string).collect(),
        fallback_rules.into_iter().map(str::to_string).collect(),
        vec![format!("phase3a_{}", family.stable_id())],
        reservable_resource.map(str::to_string),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn all_required_families_have_proposal_steps() {
        let templates = phase3a_routine_templates();
        let families = templates
            .iter()
            .map(|template| template.family)
            .collect::<BTreeSet<_>>();

        for family in [
            RoutineFamily::MorningWake,
            RoutineFamily::EatMeal,
            RoutineFamily::GoToWork,
            RoutineFamily::WorkBlock,
            RoutineFamily::ReturnHome,
            RoutineFamily::SleepNight,
            RoutineFamily::FindFood,
            RoutineFamily::ContinueCurrentIntention,
            RoutineFamily::Wait,
        ] {
            assert!(families.contains(&family), "missing {family:?}");
        }
        assert!(templates.iter().all(all_steps_are_proposals));
    }

    #[test]
    fn find_food_method_is_actor_known_only() {
        let template = phase3a_routine_templates()
            .into_iter()
            .find(|template| template.family == RoutineFamily::FindFood)
            .unwrap();

        assert!(template
            .preconditions
            .iter()
            .any(|condition| condition.contains("actor_known")));
        assert!(!template.serialize_for_test().contains("hidden"));
    }

    trait TemplateTestRender {
        fn serialize_for_test(&self) -> String;
    }

    impl TemplateTestRender for RoutineTemplate {
        fn serialize_for_test(&self) -> String {
            format!(
                "{:?}|{:?}|{:?}",
                self.applicability_conditions, self.preconditions, self.steps
            )
        }
    }
}
