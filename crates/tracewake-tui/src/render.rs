use tracewake_core::actions::report::ValidationReport;
use tracewake_core::view_models::{EmbodiedViewModel, NotebookView};

pub fn render_embodied_view(view: &EmbodiedViewModel) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "Actor: {} | Tick: {}",
        view.viewer_actor_id.as_str(),
        view.sim_tick.value()
    ));
    lines.push(format!(
        "Place: {} ({})",
        view.place_label,
        view.place_id.as_str()
    ));

    if let Some(why_not) = &view.last_rejection_why_not {
        lines.push(format!(
            "Why-not: {} kind={} reasons={}",
            why_not.actor_known_summary,
            why_not.failure_kind.stable_id(),
            why_not.reason_codes.join(",")
        ));
    } else if let Some(summary) = &view.last_rejection_summary {
        lines.push(format!("Why-not: {summary}"));
    }
    lines.push(format!(
        "Knowledge context: id={} hash={} tick={} frontier={} sources={}",
        view.holder_known_context_id.as_str(),
        view.holder_known_context_hash.as_str(),
        view.sim_tick.value(),
        view.holder_known_context_frontier,
        view.holder_known_context_source_summary
    ));
    if let Some(status) = &view.phase3a_status {
        lines.push("Needs:".to_string());
        if status.need_summaries.is_empty() {
            lines.push("- none known".to_string());
        }
        for need in &status.need_summaries {
            lines.push(format!(
                "- {}: value={} band={} cause={}",
                need.kind, need.value, need.band_label, need.last_cause
            ));
        }
        lines.push(format!(
            "Intention: {}",
            status.intention_summary.as_deref().unwrap_or("none")
        ));
        lines.push(format!(
            "Routine: {}",
            status.routine_summary.as_deref().unwrap_or("none")
        ));
        if let Some(interruption) = &status.salient_interruption {
            lines.push(format!("Interruption: {interruption}"));
        }
    }

    lines.push("Exits:".to_string());
    for exit in &view.visible_exits {
        let blocker = exit
            .blocker_summary
            .as_ref()
            .map(|summary| format!(" blocked: {summary}"))
            .unwrap_or_default();
        lines.push(format!(
            "- {}{}",
            exit.destination_place_id.as_str(),
            blocker
        ));
    }

    lines.push("Doors:".to_string());
    for door in &view.visible_doors {
        lines.push(format!(
            "- {} open={} locked={}",
            door.door_id.as_str(),
            door.is_open,
            door.is_locked
        ));
    }

    lines.push("Containers:".to_string());
    for container in &view.visible_containers {
        lines.push(format!(
            "- {} open={} locked={}",
            container.container_id.as_str(),
            container.is_open,
            container.is_locked
        ));
    }

    lines.push("Items:".to_string());
    for item in &view.visible_items {
        lines.push(format!(
            "- {} portable={}",
            item.item_id.as_str(),
            item.portable
        ));
    }

    lines.push("Inventory:".to_string());
    for item in &view.carried_items {
        lines.push(format!(
            "- {} portable={}",
            item.item_id.as_str(),
            item.portable
        ));
    }

    lines.push("Actors:".to_string());
    for actor in &view.local_actors {
        lines.push(format!("- {}", actor.actor_id.as_str()));
    }

    lines.push("Actions:".to_string());
    for (index, action) in view.semantic_actions.iter().enumerate() {
        let disabled = action
            .availability
            .actor_safe_summary()
            .map(|summary| {
                let reason_codes = action
                    .availability
                    .reason_codes()
                    .iter()
                    .map(|reason| reason.stable_id())
                    .collect::<Vec<_>>()
                    .join(",");
                format!(" disabled: {summary} reasons={reason_codes}")
            })
            .unwrap_or_default();
        lines.push(format!(
            "{}. {} [{}]{}",
            index + 1,
            action.label,
            action.semantic_action_id.as_str(),
            disabled
        ));
    }

    lines.join("\n")
}

pub fn render_notebook(view: &NotebookView) -> String {
    let mut lines = vec![format!("Notebook: {}", view.viewer_actor_id.as_str())];
    lines.push("Beliefs:".to_string());
    if view.source_bound_beliefs.is_empty() {
        lines.push("- none".to_string());
    }
    for belief in &view.source_bound_beliefs {
        let contradictions = if belief.contradiction_ids.is_empty() {
            "none".to_string()
        } else {
            belief.contradiction_ids.join(",")
        };
        lines.push(format!(
            "- {} confidence={} source={} tick={} contradictions={} :: {}",
            belief.belief_id,
            belief.confidence_label,
            belief.source_summary,
            belief.acquired_tick,
            contradictions,
            belief.summary
        ));
    }

    lines.push("Observations:".to_string());
    if view.recent_observations.is_empty() {
        lines.push("- none".to_string());
    }
    for observation in &view.recent_observations {
        lines.push(format!(
            "- {} channel={} confidence={} tick={} :: {}",
            observation.observation_id,
            observation.channel,
            observation.confidence_label,
            observation.observed_tick,
            observation.summary
        ));
    }

    lines.push("Contradictions:".to_string());
    if view.known_contradictions.is_empty() {
        lines.push("- none".to_string());
    }
    for contradiction in &view.known_contradictions {
        lines.push(format!(
            "- {} :: {}",
            contradiction.contradiction_id, contradiction.summary
        ));
    }

    lines.push("Leads:".to_string());
    if view.possible_leads.is_empty() {
        lines.push("- none".to_string());
    }
    for lead in &view.possible_leads {
        lines.push(format!("- {lead}"));
    }

    lines.join("\n")
}

pub fn render_rejection(report: &ValidationReport) -> String {
    format!(
        "Why-not: {} ({})",
        report.actor_visible_summary,
        report
            .reason_codes
            .iter()
            .map(|reason| reason.stable_id())
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::epistemics::KnowledgeContext;
    use tracewake_core::ids::{ActionId, ActorId, ItemId, PlaceId, SemanticActionId, ViewModelId};
    use tracewake_core::time::SimTick;
    use tracewake_core::view_models::{
        EmbodiedViewModel, SemanticActionEntry, ViewMode, VisibleItem, VisibleItemSource,
    };

    fn context() -> KnowledgeContext {
        KnowledgeContext::embodied(ActorId::new("actor_lina").unwrap(), SimTick::ZERO)
    }

    #[test]
    fn renderer_prints_semantic_action_ids() {
        let context = context();
        let view = EmbodiedViewModel {
            view_model_id: ViewModelId::new("view.actor_lina.0").unwrap(),
            mode: ViewMode::Embodied,
            viewer_actor_id: ActorId::new("actor_lina").unwrap(),
            sim_tick: SimTick::ZERO,
            place_id: PlaceId::new("market_stall").unwrap(),
            place_label: "Market stall".to_string(),
            visible_exits: Vec::new(),
            visible_doors: Vec::new(),
            visible_containers: Vec::new(),
            visible_items: Vec::new(),
            carried_items: Vec::new(),
            local_actors: Vec::new(),
            semantic_actions: vec![SemanticActionEntry::new(
                SemanticActionId::new("open.door.door_market_store").unwrap(),
                ActionId::new("open").unwrap(),
                vec!["door_market_store".to_string()],
                "open door_market_store",
                true,
                None,
            )],
            phase3a_status: None,
            last_rejection_summary: None,
            last_rejection_why_not: None,
            holder_known_context_id: context.holder_known_context_id().clone(),
            holder_known_context_hash: context.holder_known_context_hash().clone(),
            holder_known_context_frontier: context.event_frontier,
            holder_known_context_source_summary: "allowed=5 provenance=5".to_string(),
            notebook: None,
            debug_available: true,
        };

        let rendered = render_embodied_view(&view);

        assert!(rendered.contains("open.door.door_market_store"));
        assert!(rendered.contains("Market stall"));
    }

    #[test]
    fn renderer_prints_carried_items_under_inventory_not_items() {
        let context = context();
        let view = EmbodiedViewModel {
            view_model_id: ViewModelId::new("view.actor_lina.0").unwrap(),
            mode: ViewMode::Embodied,
            viewer_actor_id: ActorId::new("actor_lina").unwrap(),
            sim_tick: SimTick::ZERO,
            place_id: PlaceId::new("market_stall").unwrap(),
            place_label: "Market stall".to_string(),
            visible_exits: Vec::new(),
            visible_doors: Vec::new(),
            visible_containers: Vec::new(),
            visible_items: vec![VisibleItem {
                item_id: ItemId::new("loose_coin_01").unwrap(),
                source: VisibleItemSource::Place,
                portable: true,
            }],
            carried_items: vec![VisibleItem {
                item_id: ItemId::new("coin_stack_01").unwrap(),
                source: VisibleItemSource::Carried,
                portable: true,
            }],
            local_actors: Vec::new(),
            semantic_actions: Vec::new(),
            phase3a_status: None,
            last_rejection_summary: None,
            last_rejection_why_not: None,
            holder_known_context_id: context.holder_known_context_id().clone(),
            holder_known_context_hash: context.holder_known_context_hash().clone(),
            holder_known_context_frontier: context.event_frontier,
            holder_known_context_source_summary: "allowed=5 provenance=5".to_string(),
            notebook: None,
            debug_available: true,
        };

        let rendered = render_embodied_view(&view);
        let items_index = rendered.find("Items:").unwrap();
        let inventory_index = rendered.find("Inventory:").unwrap();
        let actors_index = rendered.find("Actors:").unwrap();
        let items_section = &rendered[items_index..inventory_index];
        let inventory_section = &rendered[inventory_index..actors_index];

        assert!(inventory_section.contains("- coin_stack_01 portable=true"));
        assert!(!items_section.contains("coin_stack_01"));
    }
}
