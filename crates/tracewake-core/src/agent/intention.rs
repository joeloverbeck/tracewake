use crate::ids::{ActorId, CandidateGoalId, DecisionTraceId, IntentionId, RoutineTemplateId};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntentionStatus {
    Active,
    Suspended,
    Completed,
    Failed,
    Abandoned,
    Interrupted,
}

impl IntentionStatus {
    pub const fn stable_id(self) -> &'static str {
        match self {
            IntentionStatus::Active => "active",
            IntentionStatus::Suspended => "suspended",
            IntentionStatus::Completed => "completed",
            IntentionStatus::Failed => "failed",
            IntentionStatus::Abandoned => "abandoned",
            IntentionStatus::Interrupted => "interrupted",
        }
    }

    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            IntentionStatus::Completed
                | IntentionStatus::Failed
                | IntentionStatus::Abandoned
                | IntentionStatus::Interrupted
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntentionSource {
    NeedPressure,
    RoutineDuty,
    CandidateGoalSelection,
    SafetyInterruption,
    FixtureRoutineAssignment,
    Fallback,
}

impl IntentionSource {
    pub const fn stable_id(self) -> &'static str {
        match self {
            IntentionSource::NeedPressure => "need_pressure",
            IntentionSource::RoutineDuty => "routine_duty",
            IntentionSource::CandidateGoalSelection => "candidate_goal_selection",
            IntentionSource::SafetyInterruption => "safety_interruption",
            IntentionSource::FixtureRoutineAssignment => "fixture_routine_assignment",
            IntentionSource::Fallback => "fallback",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intention {
    pub intention_id: IntentionId,
    pub actor_id: ActorId,
    pub source: IntentionSource,
    pub selected_goal_id: CandidateGoalId,
    pub selected_routine_method: Option<RoutineTemplateId>,
    pub current_step: Option<String>,
    pub durability_level: u8,
    pub start_tick: SimTick,
    pub last_progress_tick: SimTick,
    pub status: IntentionStatus,
    pub status_reason: Option<String>,
    pub trace_ancestry: Vec<DecisionTraceId>,
}

impl Intention {
    #[allow(clippy::too_many_arguments)]
    pub fn adopt(
        intention_id: IntentionId,
        actor_id: ActorId,
        source: IntentionSource,
        selected_goal_id: CandidateGoalId,
        selected_routine_method: Option<RoutineTemplateId>,
        current_step: Option<String>,
        durability_level: u8,
        start_tick: SimTick,
        trace_id: DecisionTraceId,
    ) -> Self {
        Self {
            intention_id,
            actor_id,
            source,
            selected_goal_id,
            selected_routine_method,
            current_step,
            durability_level,
            start_tick,
            last_progress_tick: start_tick,
            status: IntentionStatus::Active,
            status_reason: None,
            trace_ancestry: vec![trace_id],
        }
    }

    pub fn record_progress(&mut self, tick: SimTick, step: impl Into<String>) {
        self.last_progress_tick = tick;
        self.current_step = Some(step.into());
    }

    pub fn suspend(&mut self, reason: impl Into<String>) -> Result<(), IntentionTransitionError> {
        self.transition(IntentionStatus::Suspended, reason)
    }

    pub fn complete(&mut self, reason: impl Into<String>) -> Result<(), IntentionTransitionError> {
        self.transition(IntentionStatus::Completed, reason)
    }

    pub fn fail(&mut self, reason: impl Into<String>) -> Result<(), IntentionTransitionError> {
        self.transition(IntentionStatus::Failed, reason)
    }

    pub fn abandon(&mut self, reason: impl Into<String>) -> Result<(), IntentionTransitionError> {
        self.transition(IntentionStatus::Abandoned, reason)
    }

    pub fn interrupt(&mut self, reason: impl Into<String>) -> Result<(), IntentionTransitionError> {
        self.transition(IntentionStatus::Interrupted, reason)
    }

    pub fn reactivate(
        &mut self,
        reason: impl Into<String>,
    ) -> Result<(), IntentionTransitionError> {
        if self.status != IntentionStatus::Suspended {
            return Err(IntentionTransitionError::IllegalTransition {
                from: self.status,
                to: IntentionStatus::Active,
            });
        }

        self.status = IntentionStatus::Active;
        self.status_reason = Some(reason.into());
        Ok(())
    }

    fn transition(
        &mut self,
        to: IntentionStatus,
        reason: impl Into<String>,
    ) -> Result<(), IntentionTransitionError> {
        if self.status.is_terminal() {
            return Err(IntentionTransitionError::IllegalTransition {
                from: self.status,
                to,
            });
        }

        if self.status == to {
            return Err(IntentionTransitionError::NoopTransition {
                status: self.status,
            });
        }

        self.status = to;
        self.status_reason = Some(reason.into());
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActorIntentions {
    pub active: Option<Intention>,
    pub suspended: Vec<Intention>,
}

impl ActorIntentions {
    pub fn adopt_active(&mut self, intention: Intention) -> Result<(), IntentionTransitionError> {
        if intention.status != IntentionStatus::Active {
            return Err(IntentionTransitionError::AdoptedIntentionNotActive {
                status: intention.status,
            });
        }

        if self.active.is_some() {
            return Err(IntentionTransitionError::ActiveIntentionAlreadyExists);
        }

        self.active = Some(intention);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntentionTransitionError {
    IllegalTransition {
        from: IntentionStatus,
        to: IntentionStatus,
    },
    NoopTransition {
        status: IntentionStatus,
    },
    ActiveIntentionAlreadyExists,
    AdoptedIntentionNotActive {
        status: IntentionStatus,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intention(value: &str) -> Intention {
        Intention::adopt(
            IntentionId::new(value).unwrap(),
            ActorId::new("actor_mara").unwrap(),
            IntentionSource::CandidateGoalSelection,
            CandidateGoalId::new("goal_breakfast").unwrap(),
            Some(RoutineTemplateId::new("routine_eat_meal").unwrap()),
            Some("select_known_food".to_string()),
            5,
            SimTick::new(10),
            DecisionTraceId::new("trace_breakfast").unwrap(),
        )
    }

    #[test]
    fn active_intention_transitions_are_reason_bearing() {
        let transitions = [
            (IntentionStatus::Suspended, "resource temporarily blocked"),
            (IntentionStatus::Completed, "meal eaten"),
            (IntentionStatus::Failed, "known food missing"),
            (IntentionStatus::Abandoned, "fallback selected"),
            (IntentionStatus::Interrupted, "severe safety pressure"),
        ];

        for (status, reason) in transitions {
            let mut intention = intention(&format!("intention_{}", status.stable_id()));
            match status {
                IntentionStatus::Suspended => intention.suspend(reason).unwrap(),
                IntentionStatus::Completed => intention.complete(reason).unwrap(),
                IntentionStatus::Failed => intention.fail(reason).unwrap(),
                IntentionStatus::Abandoned => intention.abandon(reason).unwrap(),
                IntentionStatus::Interrupted => intention.interrupt(reason).unwrap(),
                IntentionStatus::Active => unreachable!(),
            }

            assert_eq!(intention.status, status);
            assert_eq!(intention.status_reason.as_deref(), Some(reason));
        }
    }

    #[test]
    fn completed_intention_cannot_be_reactivated_without_fresh_adoption() {
        let mut intention = intention("intention_complete");
        intention.complete("routine finished").unwrap();

        assert_eq!(
            intention.reactivate("start again").unwrap_err(),
            IntentionTransitionError::IllegalTransition {
                from: IntentionStatus::Completed,
                to: IntentionStatus::Active,
            }
        );
        assert_eq!(
            intention.fail("late failure").unwrap_err(),
            IntentionTransitionError::IllegalTransition {
                from: IntentionStatus::Completed,
                to: IntentionStatus::Failed,
            }
        );
    }

    #[test]
    fn suspended_intention_can_reactivate_with_reason() {
        let mut intention = intention("intention_suspend_resume");
        intention.suspend("bed occupied").unwrap();
        intention.reactivate("bed became available").unwrap();

        assert_eq!(intention.status, IntentionStatus::Active);
        assert_eq!(
            intention.status_reason.as_deref(),
            Some("bed became available")
        );
    }

    #[test]
    fn actor_intentions_allow_at_most_one_active_intention() {
        let mut intentions = ActorIntentions::default();
        intentions
            .adopt_active(intention("intention_first"))
            .unwrap();

        assert_eq!(
            intentions
                .adopt_active(intention("intention_second"))
                .unwrap_err(),
            IntentionTransitionError::ActiveIntentionAlreadyExists
        );
    }

    #[test]
    fn intention_status_stable_ids_cover_all_variants() {
        let cases = [
            (IntentionStatus::Active, "active", false),
            (IntentionStatus::Suspended, "suspended", false),
            (IntentionStatus::Completed, "completed", true),
            (IntentionStatus::Failed, "failed", true),
            (IntentionStatus::Abandoned, "abandoned", true),
            (IntentionStatus::Interrupted, "interrupted", true),
        ];

        for (status, stable_id, is_terminal) in cases {
            assert_eq!(status.stable_id(), stable_id);
            assert_eq!(status.is_terminal(), is_terminal);
        }
    }
}
