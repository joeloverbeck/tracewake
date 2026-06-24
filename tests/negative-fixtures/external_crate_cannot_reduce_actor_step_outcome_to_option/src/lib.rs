use tracewake_core::actions::Proposal;
use tracewake_core::agent::ActorDecisionTransactionOutcome;

pub fn reduce_actor_step_outcome(
    outcome: ActorDecisionTransactionOutcome,
) -> Option<Proposal> {
    outcome.into()
}
