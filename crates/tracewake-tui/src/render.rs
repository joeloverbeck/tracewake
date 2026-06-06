use tracewake_core::actions::report::ValidationReport;
use tracewake_core::view_models::EmbodiedViewModel;

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

    if let Some(summary) = &view.last_rejection_summary {
        lines.push(format!("Why-not: {summary}"));
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

    lines.push("Actors:".to_string());
    for actor in &view.local_actors {
        lines.push(format!("- {}", actor.actor_id.as_str()));
    }

    lines.push("Actions:".to_string());
    for (index, action) in view.semantic_actions.iter().enumerate() {
        let disabled = action
            .why_disabled
            .as_ref()
            .map(|reason| format!(" disabled: {reason}"))
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
    use tracewake_core::ids::{ActionId, ActorId, PlaceId, SemanticActionId, ViewModelId};
    use tracewake_core::time::SimTick;
    use tracewake_core::view_models::{EmbodiedViewModel, SemanticActionEntry, ViewMode};

    #[test]
    fn renderer_prints_semantic_action_ids() {
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
            local_actors: Vec::new(),
            semantic_actions: vec![SemanticActionEntry::new(
                SemanticActionId::new("open.door.door_market_store").unwrap(),
                ActionId::new("open").unwrap(),
                vec!["door_market_store".to_string()],
                "open door_market_store",
                true,
                None,
            )],
            last_rejection_summary: None,
            debug_available: true,
        };

        let rendered = render_embodied_view(&view);

        assert!(rendered.contains("open.door.door_market_store"));
        assert!(rendered.contains("Market stall"));
    }
}
