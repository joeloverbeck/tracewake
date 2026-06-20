use crate::agent::{
    GoalKind, RoutineCondition, RoutineFamily, RoutineStep, RoutineStepProposal, RoutineTemplate,
};
use crate::ids::{RoutineTemplateId, SemanticActionId};

pub fn phase3a_routine_templates() -> Vec<RoutineTemplate> {
    vec![
        template(
            "routine_morning_wake",
            RoutineFamily::MorningWake,
            vec![RoutineCondition::ActorKnowsSleepPlace],
            vec![RoutineCondition::SleepStateCanEnd],
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
            vec![RoutineCondition::ActorKnowsFoodSource],
            vec![RoutineCondition::FoodSourceBelievedAccessible],
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
            vec![RoutineCondition::ActorKnowsWorkplace],
            vec![RoutineCondition::KnownRouteSurface],
            vec![step("move_toward_place", "move")],
            vec!["route_blocked", "workplace_unknown"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_work_block",
            RoutineFamily::WorkBlock,
            vec![RoutineCondition::WorkplaceAssignmentActive],
            vec![RoutineCondition::ActorAtWorkplace],
            vec![step("start_work_block", "work_block")],
            vec!["workplace_closed", "need_blocked"],
            vec!["fallback_wait_with_reason"],
            Some("workstation"),
        ),
        template(
            "routine_return_home",
            RoutineFamily::ReturnHome,
            vec![RoutineCondition::ActorKnowsHome],
            vec![RoutineCondition::KnownRouteSurface],
            vec![step("move_toward_place", "move")],
            vec!["route_home_blocked"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_sleep_night",
            RoutineFamily::SleepNight,
            vec![RoutineCondition::ActorKnowsSleepPlace],
            vec![RoutineCondition::SleepPlaceBelievedAccessible],
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
            vec![RoutineCondition::ActorHasFoodSearchKnowledge],
            vec![RoutineCondition::SearchSurfaceActorKnown],
            vec![
                step("check_known_container", "check_container"),
                step("fallback_to_find_food", "eat"),
            ],
            vec!["no_known_food_sources", "search_blocked"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_leave_unsafe_place",
            RoutineFamily::LeaveUnsafePlace,
            Vec::new(),
            Vec::new(),
            vec![step("move_toward_place", "move")],
            vec!["no_actor_known_exit", "route_blocked"],
            vec!["wait_with_reason", "fallback_stuck_diagnostic"],
            None,
        ),
        template(
            "routine_continue_current_intention",
            RoutineFamily::ContinueCurrentIntention,
            vec![RoutineCondition::ActiveIntentionPresent],
            vec![RoutineCondition::NextStepAvailable],
            vec![step("continue_current_step", "continue_routine")],
            vec!["no_current_intention", "step_blocked"],
            vec!["fallback_wait_with_reason"],
            None,
        ),
        template(
            "routine_wait_idle",
            RoutineFamily::Wait,
            vec![RoutineCondition::ModeledWaitReason],
            vec![RoutineCondition::ReevaluationWindowKnown],
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
        GoalKind::LeaveUnsafePlace => RoutineFamily::LeaveUnsafePlace,
        GoalKind::ContinueCurrentIntention => RoutineFamily::ContinueCurrentIntention,
        GoalKind::IdleWithReason => RoutineFamily::Wait,
    }
}

pub fn all_steps_are_proposals(template: &RoutineTemplate) -> bool {
    template.steps.iter().all(|step| match step.proposed() {
        RoutineStepProposal::Action(action_id) => !action_id.as_str().is_empty(),
        RoutineStepProposal::Wait(reason) => !reason.is_empty(),
        RoutineStepProposal::Diagnostic(diagnostic) => !diagnostic.stable_id().is_empty(),
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

#[allow(clippy::too_many_arguments)]
fn template(
    id: &str,
    family: RoutineFamily,
    applicability_conditions: Vec<RoutineCondition>,
    preconditions: Vec<RoutineCondition>,
    steps: Vec<RoutineStep>,
    failure_modes: Vec<&str>,
    fallback_rules: Vec<&str>,
    reservable_resource: Option<&str>,
) -> RoutineTemplate {
    RoutineTemplate::new(
        RoutineTemplateId::new(id).unwrap(),
        family,
        applicability_conditions,
        preconditions,
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
            RoutineFamily::LeaveUnsafePlace,
            RoutineFamily::ContinueCurrentIntention,
            RoutineFamily::Wait,
        ] {
            assert!(families.contains(&family), "missing {family:?}");
        }
        assert!(templates.iter().all(all_steps_are_proposals));
    }

    #[test]
    fn phase3a_template_census_has_defeasible_machinery_for_each_family() {
        let templates = phase3a_routine_templates();
        let expected = [
            ("routine_morning_wake", RoutineFamily::MorningWake),
            ("routine_eat_meal", RoutineFamily::EatMeal),
            ("routine_go_to_work", RoutineFamily::GoToWork),
            ("routine_work_block", RoutineFamily::WorkBlock),
            ("routine_return_home", RoutineFamily::ReturnHome),
            ("routine_sleep_night", RoutineFamily::SleepNight),
            ("routine_find_food", RoutineFamily::FindFood),
            (
                "routine_leave_unsafe_place",
                RoutineFamily::LeaveUnsafePlace,
            ),
            (
                "routine_continue_current_intention",
                RoutineFamily::ContinueCurrentIntention,
            ),
            ("routine_wait_idle", RoutineFamily::Wait),
        ];

        assert_eq!(
            templates.len(),
            expected.len(),
            "unexpected phase3a routine template count"
        );

        for (template_id, family) in expected {
            let template = templates
                .iter()
                .find(|candidate| {
                    candidate.template_id.as_str() == template_id && candidate.family == family
                })
                .unwrap_or_else(|| panic!("missing template {template_id} / {family:?}"));
            assert!(
                !template.steps.is_empty(),
                "{template_id} must expose at least one method/proposal step"
            );
            assert!(
                all_steps_are_proposals(template),
                "{template_id} must not encode empty routine proposals"
            );
            assert!(
                !template.interruption_points.is_empty(),
                "{template_id} must expose at least one interruptor checkpoint"
            );
            assert!(
                !template.failure_modes.is_empty(),
                "{template_id} must have explicit failure modes"
            );
            assert!(
                !template.fallback_rules.is_empty(),
                "{template_id} must have explicit fallback rules"
            );
            assert!(
                template
                    .debug_labels
                    .iter()
                    .any(|label| label == &format!("phase3a_{}", family.stable_id())),
                "{template_id} must carry a trace/debug label for its family"
            );
        }
    }

    #[test]
    fn find_food_method_is_actor_known_only() {
        let template = phase3a_routine_templates()
            .into_iter()
            .find(|template| template.family == RoutineFamily::FindFood)
            .unwrap();

        assert!(template
            .preconditions
            .contains(&RoutineCondition::SearchSurfaceActorKnown));
        assert!(!template.serialize_for_test().contains("hidden"));
    }

    #[test]
    fn leave_unsafe_place_uses_flight_family_and_move_step() {
        assert_eq!(
            family_for_goal(GoalKind::LeaveUnsafePlace),
            RoutineFamily::LeaveUnsafePlace
        );

        let template = phase3a_routine_templates()
            .into_iter()
            .find(|template| template.family == RoutineFamily::LeaveUnsafePlace)
            .unwrap();

        assert_eq!(template.template_id.as_str(), "routine_leave_unsafe_place");
        assert_eq!(
            template.steps.first().unwrap().proposed(),
            RoutineStepProposal::Action(&SemanticActionId::new("move").unwrap())
        );
    }

    #[test]
    fn proposal_step_validation_rejects_empty_wait_payload() {
        let mut template = phase3a_routine_templates()
            .into_iter()
            .find(|template| template.family == RoutineFamily::Wait)
            .unwrap();
        assert!(all_steps_are_proposals(&template));

        template.steps = vec![RoutineStep::WaitUntil {
            reason: String::new(),
        }];
        assert!(!all_steps_are_proposals(&template));
    }

    #[test]
    fn proposal_step_validation_accepts_typed_diagnostic_payload() {
        let mut template = phase3a_routine_templates()
            .into_iter()
            .find(|template| template.family == RoutineFamily::Wait)
            .unwrap();
        template.steps = vec![RoutineStep::FailWithTypedDiagnostic {
            diagnostic: crate::agent::routine::RoutineDiagnosticKind::NoSleepAffordance,
        }];

        assert!(all_steps_are_proposals(&template));
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
