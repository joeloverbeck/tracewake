use crate::agent::{
    ActorKnownFact, ActorKnownProvenance, ApplicabilityResult, CandidateGoal, DecisionOutcome,
    DecisionTrace, HiddenTruthAudit, Intention, IntentionSource, RejectedDecisionItem,
};
use crate::epistemics::KnowledgeProvenanceKind;
use crate::ids::{ActorId, CandidateGoalId, DecisionTraceId, IntentionId};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActorKnownInputSourceClass {
    ActorKnown(KnowledgeProvenanceKind),
    RoutineAssignment,
    FixturePossibility,
    PipelineValidationTruth,
    DebugOmniscience,
    UnprovenPhysicalTruth,
}

impl ActorKnownInputSourceClass {
    pub const fn stable_id(&self) -> &'static str {
        match self {
            Self::ActorKnown(kind) => kind.stable_id(),
            Self::RoutineAssignment => "routine_assignment",
            Self::FixturePossibility => "fixture_possibility",
            Self::PipelineValidationTruth => "pipeline_validation_truth",
            Self::DebugOmniscience => "debug_omniscience",
            Self::UnprovenPhysicalTruth => "unproven_physical_truth",
        }
    }

    pub const fn is_forbidden_for_cognition(&self) -> bool {
        matches!(
            self,
            Self::PipelineValidationTruth | Self::DebugOmniscience | Self::UnprovenPhysicalTruth
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorKnownInputRef {
    pub fact_id: String,
    pub proposition_id: Option<String>,
    pub provenance_edge_ids: Vec<String>,
    pub source_event_ids: Vec<String>,
    pub source_class: ActorKnownInputSourceClass,
    pub confidence: Option<u16>,
    pub staleness: Option<String>,
    pub explicit_unknown: bool,
    pub display_note: String,
}

impl ActorKnownInputRef {
    pub fn from_fact(fact: &ActorKnownFact) -> Self {
        let source_class = source_class_for_provenance(fact.provenance());
        Self {
            fact_id: fact.stable_id().to_string(),
            proposition_id: Some(format!("{}:{}", fact.semantic_kind(), fact.value())),
            provenance_edge_ids: vec![fact.proof_note()],
            source_event_ids: fact
                .source_event_ids()
                .iter()
                .map(|event_id| event_id.as_str().to_string())
                .collect(),
            source_class,
            confidence: None,
            staleness: None,
            explicit_unknown: !fact.is_actor_known(),
            display_note: fact.proof_note(),
        }
    }

    pub fn render_for_trace(&self) -> String {
        let source_events = if self.source_event_ids.is_empty() {
            "-".to_string()
        } else {
            self.source_event_ids.join(",")
        };
        format!(
            "{}|source_class={}|explicit_unknown={}|source_events={}",
            self.display_note,
            self.source_class.stable_id(),
            self.explicit_unknown,
            source_events
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionInput {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub candidates: Vec<CandidateGoal>,
    pub active_intention: Option<Intention>,
    pub actor_known_inputs: Vec<ActorKnownInputRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionSelection {
    pub selected_goal: CandidateGoal,
    pub trace: DecisionTrace,
    pub lifecycle_effects: Vec<IntentionLifecycleEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntentionLifecycleEffect {
    Continued {
        intention_id: IntentionId,
        reason: String,
    },
    Interrupted {
        intention_id: IntentionId,
        reason: String,
    },
    Adopted {
        intention: Intention,
        reason: String,
    },
}

pub fn select_goal_and_trace(input: DecisionInput) -> Option<DecisionSelection> {
    let mut applicable = input
        .candidates
        .iter()
        .filter(|candidate| candidate.applicability == ApplicabilityResult::Applicable)
        .cloned()
        .collect::<Vec<_>>();
    applicable.sort();
    let selected = applicable.into_iter().next()?;
    let active_intention = input.active_intention.clone();
    let outcome = if selected.goal_kind == crate::agent::GoalKind::ContinueCurrentIntention {
        DecisionOutcome::Continued
    } else if active_intention.is_some() {
        DecisionOutcome::Switched
    } else {
        DecisionOutcome::Continued
    };

    let trace_id = DecisionTraceId::new(format!(
        "trace_decision_{}_{}_{}",
        input.actor_id.as_str(),
        input.decision_tick.value(),
        selected.candidate_goal_id.as_str()
    ))
    .unwrap();
    let lifecycle_effects = lifecycle_effects(
        &input.actor_id,
        input.decision_tick,
        &selected,
        active_intention.as_ref(),
        &trace_id,
    );
    let rejected_goals = input
        .candidates
        .iter()
        .filter(|candidate| candidate.candidate_goal_id != selected.candidate_goal_id)
        .map(|candidate| RejectedDecisionItem {
            stable_ref: candidate.candidate_goal_id.to_string(),
            reason: candidate
                .rejection_reason
                .clone()
                .unwrap_or_else(|| "lower_priority".to_string()),
        })
        .collect();
    let hidden_truth_audit = hidden_truth_audit_from_actor_known_inputs(&input.actor_known_inputs);
    let actor_known_inputs = input
        .actor_known_inputs
        .iter()
        .map(ActorKnownInputRef::render_for_trace)
        .collect();
    let trace = DecisionTrace::new(
        trace_id,
        input.actor_id,
        input.decision_tick,
        input.decision_tick.advance_by(1),
        Vec::new(),
        active_intention,
        input.candidates,
        Some(selected.candidate_goal_id.clone()),
        selected.selected_routine_method.clone(),
        None,
        rejected_goals,
        Vec::new(),
        Vec::new(),
        actor_known_inputs,
        None,
        Some("selected_by_priority_order".to_string()),
        None,
        hidden_truth_audit,
        outcome,
        "deterministic candidate selection".to_string(),
    );
    Some(DecisionSelection {
        selected_goal: selected,
        trace,
        lifecycle_effects,
    })
}

fn hidden_truth_audit_from_actor_known_inputs(
    actor_known_inputs: &[ActorKnownInputRef],
) -> HiddenTruthAudit {
    let actor_known_only = actor_known_inputs
        .iter()
        .all(|input| !input.source_class.is_forbidden_for_cognition() && !input.explicit_unknown);
    let forbidden_count = actor_known_inputs
        .iter()
        .filter(|input| input.source_class.is_forbidden_for_cognition() || input.explicit_unknown)
        .count();
    HiddenTruthAudit {
        actor_known_only,
        notes: format!(
            "candidate inputs supplied by actor-known generation boundary: count={} forbidden_source_classes={}",
            actor_known_inputs.len(),
            forbidden_count
        ),
    }
}

fn source_class_for_provenance(provenance: &ActorKnownProvenance) -> ActorKnownInputSourceClass {
    match provenance {
        ActorKnownProvenance::ObservedNow { .. } => {
            ActorKnownInputSourceClass::ActorKnown(KnowledgeProvenanceKind::Observation)
        }
        ActorKnownProvenance::RememberedBelief { .. } => {
            ActorKnownInputSourceClass::ActorKnown(KnowledgeProvenanceKind::Memory)
        }
        ActorKnownProvenance::RoutineAssignment { .. } => {
            ActorKnownInputSourceClass::RoutineAssignment
        }
        ActorKnownProvenance::FixturePossibility { .. } => {
            ActorKnownInputSourceClass::FixturePossibility
        }
        ActorKnownProvenance::PipelineValidationTruth(_) => {
            ActorKnownInputSourceClass::PipelineValidationTruth
        }
        ActorKnownProvenance::DebugOmniscience(_) => ActorKnownInputSourceClass::DebugOmniscience,
        ActorKnownProvenance::UnprovenPhysicalTruth { .. } => {
            ActorKnownInputSourceClass::UnprovenPhysicalTruth
        }
    }
}

pub fn selected_goal_id(selection: &DecisionSelection) -> &CandidateGoalId {
    &selection.selected_goal.candidate_goal_id
}

fn lifecycle_effects(
    actor_id: &ActorId,
    decision_tick: SimTick,
    selected: &CandidateGoal,
    active_intention: Option<&Intention>,
    trace_id: &DecisionTraceId,
) -> Vec<IntentionLifecycleEffect> {
    if selected.goal_kind == crate::agent::GoalKind::ContinueCurrentIntention {
        return active_intention
            .map(|intention| {
                vec![IntentionLifecycleEffect::Continued {
                    intention_id: intention.intention_id.clone(),
                    reason: "selected active intention continuation candidate".to_string(),
                }]
            })
            .unwrap_or_default();
    }

    let mut effects = Vec::new();
    if let Some(intention) = active_intention {
        effects.push(IntentionLifecycleEffect::Interrupted {
            intention_id: intention.intention_id.clone(),
            reason: format!(
                "interrupted by {} candidate {}",
                selected.priority.stable_id(),
                selected.candidate_goal_id.as_str()
            ),
        });
    }

    effects.push(IntentionLifecycleEffect::Adopted {
        intention: Intention::adopt(
            IntentionId::new(format!(
                "intention_{}_{}_{}",
                actor_id.as_str(),
                decision_tick.value(),
                selected.goal_kind.stable_id()
            ))
            .unwrap(),
            actor_id.clone(),
            intention_source_for(selected),
            selected.candidate_goal_id.clone(),
            selected.selected_routine_method.clone(),
            Some(selected.goal_kind.stable_id().to_string()),
            durability_level_for(selected),
            decision_tick,
            trace_id.clone(),
        ),
        reason: "selected candidate becomes active intention".to_string(),
    });
    effects
}

fn intention_source_for(selected: &CandidateGoal) -> IntentionSource {
    match selected.source {
        crate::agent::CandidateGoalSource::NeedPressure => IntentionSource::NeedPressure,
        crate::agent::CandidateGoalSource::RoutineDuty
        | crate::agent::CandidateGoalSource::FixtureRoutineAssignment => {
            IntentionSource::RoutineDuty
        }
        crate::agent::CandidateGoalSource::SafetyInterruption => {
            IntentionSource::SafetyInterruption
        }
        crate::agent::CandidateGoalSource::CurrentIntentionContinuation => {
            IntentionSource::CandidateGoalSelection
        }
        crate::agent::CandidateGoalSource::Fallback => IntentionSource::Fallback,
    }
}

fn durability_level_for(selected: &CandidateGoal) -> u8 {
    match selected.priority {
        crate::agent::GoalPriority::SevereSafety
        | crate::agent::GoalPriority::SevereHunger
        | crate::agent::GoalPriority::SevereFatigue => 9,
        crate::agent::GoalPriority::ActiveIntentionContinuation => 8,
        crate::agent::GoalPriority::UrgentHungerOrFatigue => 7,
        crate::agent::GoalPriority::RoutineWindowDuty
        | crate::agent::GoalPriority::ReturnHomeOrSleepWindow => 6,
        crate::agent::GoalPriority::IdleWithReason
        | crate::agent::GoalPriority::MildNeedPressure => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::generation::{generate_candidate_goals, CandidateGenerationInput};
    use crate::agent::{
        ActorKnownFact, GoalKind, Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState,
        SourceEventIds,
    };
    use crate::epistemics::KnowledgeProvenanceKind;
    use crate::ids::{CandidateGoalId, EventId, IntentionId, RoutineTemplateId};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn test_source() -> SourceEventIds {
        SourceEventIds::checked(vec![EventId::new("event_test_actor_known").unwrap()]).unwrap()
    }

    fn active_work_intention() -> Intention {
        Intention::adopt(
            IntentionId::new("intention_work").unwrap(),
            actor_id(),
            IntentionSource::RoutineDuty,
            CandidateGoalId::new("goal_work").unwrap(),
            Some(RoutineTemplateId::new("routine_work").unwrap()),
            Some("work_block".to_string()),
            8,
            SimTick::new(10),
            DecisionTraceId::new("trace_existing").unwrap(),
        )
    }

    fn decide_for_hunger(value: i32) -> DecisionSelection {
        let active = active_work_intention();
        let generated = generate_candidate_goals(&CandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            needs: vec![NeedState::initial(
                NeedKind::Hunger,
                value,
                NeedChangeCause::TickDelta,
            )],
            active_intention: Some(active.clone()),
            actor_known_facts: vec![ActorKnownFact::observed_now(
                actor_id(),
                "actor_knows_food_source",
                "food_soup",
                "test:visible_food",
                None,
                test_source(),
            )],
            routine_window_goal: None,
        });
        select_goal_and_trace(DecisionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            candidates: generated.candidates,
            active_intention: Some(active),
            actor_known_inputs: generated.actor_known_inputs_used,
        })
        .unwrap()
    }

    fn candidate(priority: crate::agent::GoalPriority) -> CandidateGoal {
        CandidateGoal::new(
            CandidateGoalId::new(format!("goal_{}", priority.stable_id())).unwrap(),
            actor_id(),
            SimTick::new(12),
            SimTick::new(13),
            crate::agent::CandidateGoalSource::NeedPressure,
            GoalKind::Eat,
            priority,
            priority.stable_id(),
            Vec::new(),
            ApplicabilityResult::Applicable,
            None,
            None,
            DecisionTraceId::new(format!("trace_{}", priority.stable_id())).unwrap(),
        )
    }

    #[test]
    fn actor_known_input_source_class_stable_ids_cover_all_variants() {
        let cases = [
            (
                ActorKnownInputSourceClass::ActorKnown(KnowledgeProvenanceKind::Observation),
                "observation",
            ),
            (
                ActorKnownInputSourceClass::ActorKnown(KnowledgeProvenanceKind::Memory),
                "memory",
            ),
            (
                ActorKnownInputSourceClass::RoutineAssignment,
                "routine_assignment",
            ),
            (
                ActorKnownInputSourceClass::FixturePossibility,
                "fixture_possibility",
            ),
            (
                ActorKnownInputSourceClass::PipelineValidationTruth,
                "pipeline_validation_truth",
            ),
            (
                ActorKnownInputSourceClass::DebugOmniscience,
                "debug_omniscience",
            ),
            (
                ActorKnownInputSourceClass::UnprovenPhysicalTruth,
                "unproven_physical_truth",
            ),
        ];

        assert_eq!(
            cases
                .iter()
                .map(|(source_class, _)| source_class.stable_id())
                .collect::<Vec<_>>(),
            cases
                .iter()
                .map(|(_, stable_id)| *stable_id)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn durability_levels_distinguish_priority_bands() {
        use crate::agent::GoalPriority;

        assert_eq!(
            durability_level_for(&candidate(GoalPriority::SevereSafety)),
            9
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::SevereHunger)),
            9
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::SevereFatigue)),
            9
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::ActiveIntentionContinuation)),
            8
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::UrgentHungerOrFatigue)),
            7
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::RoutineWindowDuty)),
            6
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::ReturnHomeOrSleepWindow)),
            6
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::IdleWithReason)),
            4
        );
        assert_eq!(
            durability_level_for(&candidate(GoalPriority::MildNeedPressure)),
            4
        );
    }

    #[test]
    fn mild_hunger_continues_active_intention() {
        let selection = decide_for_hunger(300);

        assert_eq!(
            selection.selected_goal.goal_kind,
            GoalKind::ContinueCurrentIntention
        );
        assert_eq!(selection.trace.outcome, DecisionOutcome::Continued);
        assert_eq!(
            selection.lifecycle_effects,
            vec![IntentionLifecycleEffect::Continued {
                intention_id: IntentionId::new("intention_work").unwrap(),
                reason: "selected active intention continuation candidate".to_string(),
            }]
        );
        assert!(selection.trace.hidden_truth_audit_result.actor_known_only);
    }

    #[test]
    fn severe_hunger_switches_and_emits_trace() {
        let selection = decide_for_hunger(800);

        assert_eq!(selection.selected_goal.goal_kind, GoalKind::Eat);
        assert_eq!(selection.trace.outcome, DecisionOutcome::Switched);
        assert!(matches!(
            &selection.lifecycle_effects[..],
            [
                IntentionLifecycleEffect::Interrupted {
                    intention_id,
                    reason
                },
                IntentionLifecycleEffect::Adopted {
                    intention,
                    reason: adoption_reason
                }
            ] if intention_id.as_str() == "intention_work"
                && reason.contains("severe_hunger")
                && intention.source == IntentionSource::NeedPressure
                && intention.selected_goal_id == selection.selected_goal.candidate_goal_id
                && intention.trace_ancestry == vec![selection.trace.trace_id.clone()]
                && adoption_reason == "selected candidate becomes active intention"
        ));
        assert_eq!(
            selection.trace.selected_goal_id,
            Some(selection.selected_goal.candidate_goal_id.clone())
        );
        assert!(selection
            .trace
            .rejected_goals
            .iter()
            .all(|rejected| rejected.stable_ref
                != selection.selected_goal.candidate_goal_id.as_str()));
        assert!(selection.trace.rejected_goals.iter().any(
            |rejected| rejected.stable_ref == "goal_actor_tomas_12_continue_current_intention"
        ));
    }

    #[test]
    fn hidden_truth_audit_rejects_forbidden_provenance_without_banned_words() {
        let forbidden_input = ActorKnownInputRef {
            fact_id: "actor_knows_food_source".to_string(),
            proposition_id: Some("observed_now:food_basket".to_string()),
            provenance_edge_ids: vec!["edge_fixture_supplied".to_string()],
            source_event_ids: Vec::new(),
            source_class: ActorKnownInputSourceClass::DebugOmniscience,
            confidence: None,
            staleness: None,
            explicit_unknown: false,
            display_note: "actor_knows_food_source=visible basket".to_string(),
        };

        assert!(!forbidden_input.display_note.contains("unproven"));
        assert!(!forbidden_input.display_note.contains("debug_omniscience"));
        assert!(!forbidden_input.display_note.contains("physical_truth"));

        let audit = hidden_truth_audit_from_actor_known_inputs(&[forbidden_input]);

        assert!(!audit.actor_known_only);
        assert!(audit.notes.contains("forbidden_source_classes=1"));
    }

    #[test]
    fn repeated_selection_is_deterministic() {
        let first = decide_for_hunger(800);
        let second = decide_for_hunger(800);

        assert_eq!(selected_goal_id(&first), selected_goal_id(&second));
        assert_eq!(
            first.trace.serialize_canonical(),
            second.trace.serialize_canonical()
        );
    }
}
