use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::epistemics::{
    actor_has_source_backed_support, EpistemicProjection, KnowledgeContext, Proposition,
};
use crate::ids::ActorId;
use crate::state::PhysicalState;

pub fn validate_truthful_accuse_probe(
    state: &PhysicalState,
    projection: Option<&EpistemicProjection>,
    proposal: &Proposal,
) -> Result<(), ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    let actor = state.actors.get(&actor_id).ok_or_else(actor_missing)?;
    let accused_actor_id = accused_target(proposal)?;
    if !state.actors.contains_key(&accused_actor_id) {
        return Err(reject(
            ReasonCode::TargetNotFound,
            "accused_actor_id",
            accused_actor_id.as_str(),
            "That person is not known here.",
            "truthful accuse probe target actor was missing",
        ));
    }

    let proposition = Proposition::ActorWasNearPlace {
        actor_id: accused_actor_id.clone(),
        place_id: actor.current_place_id.clone(),
    };
    let supported = projection
        .map(|projection| {
            let context = KnowledgeContext::embodied(actor_id.clone(), proposal.requested_tick);
            actor_has_source_backed_support(&context, projection, &proposition)
        })
        .unwrap_or(false);

    if supported {
        Ok(())
    } else {
        Err(reject(
            ReasonCode::KnowledgePreconditionNotMet,
            "knowledge_basis",
            "missing_source_backed_support",
            "This actor has no source-backed observation linking anyone to the missing property.",
            format!(
                "debug: {} lacks source-backed support for {}",
                actor_id.as_str(),
                proposition.render()
            ),
        ))
    }
}

fn accused_target(proposal: &Proposal) -> Result<ActorId, ActionRejection> {
    let target = proposal.target_ids.first().ok_or_else(|| {
        reject(
            ReasonCode::TargetNotFound,
            "target_count",
            &proposal.target_ids.len().to_string(),
            "There is no person to evaluate.",
            "truthful_accuse_probe proposal did not include an actor target",
        )
    })?;
    ActorId::new(target).map_err(|_| {
        reject(
            ReasonCode::UnsupportedTargetKind,
            "accused_actor_id",
            target,
            "That person identifier is invalid.",
            "truthful_accuse_probe target was not a stable actor ID",
        )
    })
}

fn actor_missing() -> ActionRejection {
    ActionRejection::new(
        PipelineStage::ActorLookup,
        ReasonCode::ActorNotFound,
        Vec::new(),
        "That actor cannot act.",
        "actor was missing",
    )
}

fn reject(
    reason_code: ReasonCode,
    key: &'static str,
    value: &str,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> ActionRejection {
    ActionRejection::new(
        PipelineStage::KnowledgePerceptionPlaceholder,
        reason_code,
        vec![CheckedFact::new(key, value)],
        actor_visible_summary,
        debug_summary,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::proposal::ProposalOrigin;
    use crate::ids::{ActionId, PlaceId, ProposalId};
    use crate::state::{ActorBody, PhysicalState, PlaceState};
    use crate::time::SimTick;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        state.places.insert(
            PlaceId::new("shop_front").unwrap(),
            PlaceState::new(PlaceId::new("shop_front").unwrap(), "Shop front"),
        );
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(actor_id("actor_tomas"), PlaceId::new("shop_front").unwrap()),
        );
        state.actors.insert(
            actor_id("actor_mara"),
            ActorBody::new(actor_id("actor_mara"), PlaceId::new("shop_front").unwrap()),
        );
        state
    }

    fn proposal() -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_accuse_probe").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id("actor_tomas")),
            ActionId::new("truthful_accuse_probe").unwrap(),
            SimTick::ZERO,
        );
        proposal.target_ids.push("actor_mara".to_string());
        proposal
    }

    #[test]
    fn unsupported_probe_rejects_without_culprit_leak_in_embodied_summary() {
        let rejection = validate_truthful_accuse_probe(&state(), None, &proposal()).unwrap_err();

        assert_eq!(
            rejection.reason_code,
            ReasonCode::KnowledgePreconditionNotMet
        );
        assert!(!rejection.actor_visible_summary.contains("actor_mara"));
        assert!(!rejection.actor_visible_summary.contains("stole"));
        assert!(rejection.debug_summary.contains("actor_mara"));
    }
}
