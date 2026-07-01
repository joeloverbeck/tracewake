use tracewake_core::actions::report::ValidationReport;
use tracewake_core::view_models::VisibleItemSource;
use tracewake_core::view_models::{EmbodiedViewModel, NotebookView};

use crate::screen::text_dump::render_actor_line;

pub const DEBUG_TOKENS: &[&str] = &[
    "DEBUG NON-DIEGETIC",
    "Knowledge context",
    "Debug:",
    "debug_diagnostics",
];

#[deny(unused_variables)]
pub fn render_embodied_view(view: &EmbodiedViewModel) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Actor: {}", view.viewer_actor_id().as_str()));
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
        if !why_not.actor_visible_facts.is_empty() {
            lines.push(format!(
                "Why-not facts: {}",
                why_not.actor_visible_facts.join(",")
            ));
        }
    } else if let Some(summary) = &view.last_rejection_summary {
        lines.push(format!("Why-not: {summary}"));
    }
    if let Some(status) = &view.phase3a_status {
        lines.push("Needs:".to_string());
        if status.need_summaries.is_empty() {
            lines.push("- none known".to_string());
        }
        for need in &status.need_summaries {
            lines.push(format!(
                "- {}: band={} cause={}",
                need.kind, need.band_label, need.last_cause
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
    if let Some(summary) = view.actor_known_interval_summary() {
        lines.push("Recent interval: actor-known update".to_string());
        if summary.no_new_actor_known_information() {
            lines.push("- no new actor-known notices or observations".to_string());
        }
        for notice in summary.notices() {
            lines.push(format!("- {}", notice.notice_kind().stable_id()));
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
            "- {} open={} locked={} endpoints={}<->{}",
            door.door_id.as_str(),
            door.is_open,
            door.is_locked,
            door.endpoint_a.as_str(),
            door.endpoint_b.as_str()
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
            "- {} portable={} source={}",
            item.item_id.as_str(),
            item.portable,
            visible_item_source_label(&item.source)
        ));
    }

    lines.push("Inventory:".to_string());
    for item in &view.carried_items {
        lines.push(format!(
            "- {} portable={} source={}",
            item.item_id.as_str(),
            item.portable,
            visible_item_source_label(&item.source)
        ));
    }

    lines.push("Actors:".to_string());
    for actor in &view.local_actors {
        lines.push(render_actor_line(actor));
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

pub fn render_debug_overlay(view: &EmbodiedViewModel) -> String {
    let mut lines = Vec::new();
    if !view.debug_available() {
        return lines.join("\n");
    }
    lines.push(format!("{}: Embodied Overlay", DEBUG_TOKENS[0]));
    lines.push(format!(
        "{}: id={} hash={} tick={} frontier={} sources={}",
        DEBUG_TOKENS[1],
        view.holder_known_context_id().as_str(),
        view.holder_known_context_hash().as_str(),
        view.sim_tick().value(),
        view.holder_known_context_frontier(),
        view.holder_known_context_source_summary()
    ));
    lines.push(format!(
        "{} available={}",
        DEBUG_TOKENS[2],
        view.debug_available()
    ));
    for action in &view.semantic_actions {
        let diagnostics = action.availability.debug_only_diagnostics();
        if !diagnostics.is_empty() {
            lines.push(format!(
                "{} action={} values={}",
                DEBUG_TOKENS[3],
                action.semantic_action_id.as_str(),
                diagnostics.join(",")
            ));
        }
    }
    lines.join("\n")
}

fn visible_item_source_label(source: &VisibleItemSource) -> String {
    match source {
        VisibleItemSource::Place => "place".to_string(),
        VisibleItemSource::Container(container_id) => {
            format!("container:{}", container_id.as_str())
        }
        VisibleItemSource::Carried => "carried".to_string(),
    }
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
    if view.typed_leads.is_empty() && view.possible_leads.is_empty() {
        lines.push("- none".to_string());
    }
    for lead in &view.typed_leads {
        lines.push(format!(
            "- {} contradiction={} belief={} observation={} source_kind={} source={} confidence={} detected_tick={} staleness={} wrong_if={} next_actions={} :: {}",
            lead.lead_id,
            lead.contradiction_id,
            lead.belief_id,
            lead.observation_id,
            lead.source_kind,
            lead.source_summary,
            lead.confidence_label,
            lead.detected_tick,
            lead.staleness_label,
            lead.how_this_may_be_wrong,
            lead.possible_next_actions.join(","),
            lead.summary
        ));
    }
    if view.typed_leads.is_empty() {
        for lead in &view.possible_leads {
            lines.push(format!("- {lead}"));
        }
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
    use tracewake_core::actions::report::ReasonCode;
    use tracewake_core::epistemics::KnowledgeContext;
    use tracewake_core::ids::{
        ActionId, ActorId, DoorId, ItemId, PlaceId, SemanticActionId, ViewModelId,
    };
    use tracewake_core::projections::IntervalStopReason;
    use tracewake_core::time::SimTick;
    use tracewake_core::view_models::{
        ActionAvailability, ActorKnownActivitySourceKind, EmbodiedViewModel, NotebookLeadEntry,
        NotebookView, ObservedActivityView, ObservedActorActivityKind, Phase3AEmbodiedStatus,
        SemanticActionEntry, TypedActorKnownIntervalSummary, ViewMode, VisibleActor, VisibleDoor,
        VisibleExit, VisibleItem, VisibleItemSource, WhyNotFailureKind, WhyNotView,
    };

    fn context() -> KnowledgeContext {
        KnowledgeContext::embodied(ActorId::new("actor_lina").unwrap(), SimTick::ZERO)
    }

    fn minimal_view_with_interval(
        actor_known_interval_summary: Option<TypedActorKnownIntervalSummary>,
    ) -> EmbodiedViewModel {
        let context = context();
        EmbodiedViewModel::for_test(
            ViewModelId::new("view.actor_lina.0").unwrap(),
            ViewMode::Embodied,
            ActorId::new("actor_lina").unwrap(),
            SimTick::new(4),
            PlaceId::new("market_stall").unwrap(),
            "Market stall".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            None,
            context.holder_known_context_id().clone(),
            context.holder_known_context_hash().clone(),
            context.event_frontier(),
            "allowed=5 provenance=5".to_string(),
            actor_known_interval_summary,
            None,
            true,
        )
    }

    #[test]
    fn renderer_prints_actor_known_interval_summary() {
        let no_new = render_embodied_view(&minimal_view_with_interval(Some(
            TypedActorKnownIntervalSummary::for_test(
                SimTick::ZERO,
                SimTick::new(2),
                0,
                2,
                IntervalStopReason::ControllerSafetyBound,
                Vec::new(),
                true,
            ),
        )));
        assert!(no_new.contains("Recent interval: actor-known update"));
        assert!(!no_new.contains("controller_safety_bound"));
        assert!(!no_new.contains("ticks 0-2"));
        assert!(no_new.contains("- no new actor-known notices or observations"));
        assert!(!no_new.contains("nothing happened"));

        let with_notice = render_embodied_view(&minimal_view_with_interval(Some(
            TypedActorKnownIntervalSummary::for_test(
                SimTick::ZERO,
                SimTick::new(4),
                0,
                4,
                IntervalStopReason::PossessedDurationTerminal,
                Vec::new(),
                true,
            ),
        )));
        assert!(with_notice.contains("Recent interval: actor-known update"));
        assert!(!with_notice.contains("possessed_duration_terminal"));
        assert!(!with_notice.contains("ticks 0-4"));
        assert!(with_notice.contains("- no new actor-known notices or observations"));
    }

    #[test]
    fn renderer_prints_semantic_action_ids() {
        let mut view = minimal_view_with_interval(None);
        view.semantic_actions = vec![SemanticActionEntry::new(
            SemanticActionId::new("open.door.door_market_store").unwrap(),
            ActionId::new("open").unwrap(),
            vec!["door_market_store".to_string()],
            "open door_market_store",
            true,
            None,
        )];

        let rendered = render_embodied_view(&view);

        assert!(rendered.contains("open.door.door_market_store"));
        assert!(rendered.contains("Market stall"));
        assert!(!rendered.contains("Debug: available=true"));
    }

    #[test]
    fn renderer_prints_actor_known_activity_from_visible_actor() {
        let mut view = minimal_view_with_interval(None);
        view.local_actors = vec![VisibleActor {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            display_label: "Tomas".to_string(),
            presence_source_summary: "event:event_visible_actor_tomas".to_string(),
            presence_observed_tick: SimTick::new(4),
            presence_staleness_label: "current".to_string(),
            presence_uncertainty_label: None,
            observed_activity: Some(ObservedActivityView {
                kind: ObservedActorActivityKind::Working,
                actor_safe_summary: "working at bench".to_string(),
                source: ActorKnownActivitySourceKind::DirectPerception,
                source_summary: "event:event_visible_actor_tomas".to_string(),
                observed_tick: SimTick::new(4),
                staleness_label: "current".to_string(),
                uncertainty_label: None,
            }),
        }];

        let rendered = render_embodied_view(&view);

        assert!(rendered.contains("- Tomas (actor_tomas)"));
        assert!(rendered.contains("activity=working"));
        assert!(rendered.contains("activity_source=direct perception"));
        assert!(rendered.contains("presence_source=direct perception"));
        assert!(!rendered.contains("workplace_tomas"));
        assert!(!rendered.contains("event_visible_actor_tomas"));
    }

    #[test]
    fn renderer_prints_door_endpoints_and_item_sources() {
        let mut view = minimal_view_with_interval(None);
        view.visible_doors = vec![VisibleDoor {
            door_id: DoorId::new("door_market_store").unwrap(),
            endpoint_a: PlaceId::new("market_stall").unwrap(),
            endpoint_b: PlaceId::new("store_room").unwrap(),
            is_open: false,
            is_locked: true,
        }];
        view.visible_items = vec![VisibleItem {
            item_id: ItemId::new("apple_01").unwrap(),
            source: VisibleItemSource::Container("crate_01".parse().unwrap()),
            portable: true,
        }];
        view.carried_items = vec![VisibleItem {
            item_id: ItemId::new("coin_01").unwrap(),
            source: VisibleItemSource::Carried,
            portable: true,
        }];

        let rendered = render_embodied_view(&view);

        assert!(rendered.contains(
            "- door_market_store open=false locked=true endpoints=market_stall<->store_room"
        ));
        assert!(rendered.contains("- apple_01 portable=true source=container:crate_01"));
        assert!(rendered.contains("- coin_01 portable=true source=carried"));
    }

    #[test]
    fn renderer_prints_debug_only_action_diagnostics_when_present() {
        let mut view = minimal_view_with_interval(None);
        view.semantic_actions = vec![SemanticActionEntry::with_availability(
            SemanticActionId::new("open.door_market_store").unwrap(),
            ActionId::new("open").unwrap(),
            vec!["door_market_store".to_string()],
            "open door_market_store",
            ActionAvailability::disabled(
                vec![ReasonCode::DoorClosedBlocksMovement],
                "The door is closed.",
                Vec::new(),
                vec!["validator_fact=door_closed".to_string()],
            ),
        )];

        let rendered = render_embodied_view(&view);
        let overlay = render_debug_overlay(&view);

        assert!(!rendered.contains("debug_diagnostics"));
        assert!(overlay.contains("debug_diagnostics action=open.door_market_store"));
        assert!(overlay.contains("validator_fact=door_closed"));
    }

    #[test]
    fn renderer_prints_typed_why_not_facts_from_view_model() {
        let mut view = minimal_view_with_interval(None);
        view.last_rejection_summary = Some("fallback summary must not win".to_string());
        view.last_rejection_why_not = Some(WhyNotView {
            failure_kind: WhyNotFailureKind::Access,
            actor_known_summary: "Door door_market_store is closed.".to_string(),
            reason_codes: vec!["door_closed_blocks_movement".to_string()],
            actor_visible_facts: vec![
                "door_id=door_market_store".to_string(),
                "to_place_id=store_room".to_string(),
            ],
        });

        let rendered = render_embodied_view(&view);

        assert!(rendered.contains(
            "Why-not: Door door_market_store is closed. kind=access reasons=door_closed_blocks_movement"
        ));
        assert!(
            rendered.contains("Why-not facts: door_id=door_market_store,to_place_id=store_room")
        );
        assert!(!rendered.contains("fallback summary must not win"));
    }

    #[test]
    fn render_notebook_prints_typed_lead_anatomy() {
        let view = NotebookView {
            viewer_actor_id: ActorId::new("actor_lina").unwrap(),
            source_bound_beliefs: Vec::new(),
            recent_observations: Vec::new(),
            known_contradictions: Vec::new(),
            typed_leads: vec![NotebookLeadEntry {
                lead_id: "lead_01".to_string(),
                contradiction_id: "contradiction_01".to_string(),
                belief_id: "belief_01".to_string(),
                observation_id: "observation_01".to_string(),
                source_kind: "observation".to_string(),
                source_summary: "source_event=event_01".to_string(),
                confidence_label: "700".to_string(),
                detected_tick: 8,
                staleness_label: "fresh".to_string(),
                how_this_may_be_wrong: "the source may be stale".to_string(),
                possible_next_actions: vec!["inspect place".to_string(), "ask actor".to_string()],
                summary: "The belief and observation disagree.".to_string(),
            }],
            possible_leads: Vec::new(),
        };

        let rendered = render_notebook(&view);

        assert!(rendered.contains("lead_01 contradiction=contradiction_01"));
        assert!(rendered.contains("staleness=fresh"));
        assert!(rendered.contains("wrong_if=the source may be stale"));
        assert!(rendered.contains("next_actions=inspect place,ask actor"));
        assert!(!rendered
            .lines()
            .skip_while(|line| *line != "Leads:")
            .any(|line| line == "- none"));
        assert!(!rendered.contains("legacy summary"));
    }

    #[test]
    fn renderer_prints_visible_exit_blocker_summary() {
        let mut view = minimal_view_with_interval(None);
        view.visible_exits = vec![VisibleExit {
            destination_place_id: PlaceId::new("store_room").unwrap(),
            blocker_summary: Some("door door_market_store is closed and locked".to_string()),
        }];

        let rendered = render_embodied_view(&view);

        assert!(
            rendered.contains("- store_room blocked: door door_market_store is closed and locked")
        );
    }

    #[test]
    fn renderer_prints_carried_items_under_inventory_not_items() {
        let mut view = minimal_view_with_interval(None);
        view.visible_items = vec![VisibleItem {
            item_id: ItemId::new("loose_coin_01").unwrap(),
            source: VisibleItemSource::Place,
            portable: true,
        }];
        view.carried_items = vec![VisibleItem {
            item_id: ItemId::new("coin_stack_01").unwrap(),
            source: VisibleItemSource::Carried,
            portable: true,
        }];

        let rendered = render_embodied_view(&view);
        let items_index = rendered.find("Items:").unwrap();
        let inventory_index = rendered.find("Inventory:").unwrap();
        let actors_index = rendered.find("Actors:").unwrap();
        let items_section = &rendered[items_index..inventory_index];
        let inventory_section = &rendered[inventory_index..actors_index];

        assert!(inventory_section.contains("- coin_stack_01 portable=true"));
        assert!(!items_section.contains("coin_stack_01"));
    }

    #[test]
    fn renderer_prints_phase3a_salient_interruption() {
        let mut view = minimal_view_with_interval(None);
        view.phase3a_status = Some(Phase3AEmbodiedStatus {
            need_summaries: Vec::new(),
            intention_summary: None,
            routine_summary: None,
            salient_interruption: Some(
                "sleep_interrupted at tick 8: Sleep interrupted by hunger".to_string(),
            ),
        });

        let rendered = render_embodied_view(&view);

        assert!(rendered
            .contains("Interruption: sleep_interrupted at tick 8: Sleep interrupted by hunger"));
    }

    #[test]
    fn renderer_keeps_debug_tokens_out_of_embodied_view() {
        let view = minimal_view_with_interval(None);

        let rendered = render_embodied_view(&view);

        assert_embodied_render_excludes_debug_tokens(&rendered, DEBUG_TOKENS);
    }

    #[test]
    fn renderer_debug_token_negative_extends_from_token_source() {
        let view = minimal_view_with_interval(None);
        let rendered = render_embodied_view(&view);
        let tokens = DEBUG_TOKENS
            .iter()
            .copied()
            .chain(["synthetic_debug_token"])
            .collect::<Vec<_>>();

        assert_embodied_render_excludes_debug_tokens(&rendered, &tokens);
    }

    #[test]
    fn debug_overlay_marks_knowledge_context_frontier_non_diegetic() {
        let view = minimal_view_with_interval(None);

        let rendered = render_debug_overlay(&view);

        assert!(rendered.contains("DEBUG NON-DIEGETIC: Embodied Overlay"));
        assert!(rendered.contains("Knowledge context: id=hkc."));
        assert!(rendered.contains("frontier=0"));
        assert!(rendered.contains("Debug: available=true"));
    }

    fn assert_embodied_render_excludes_debug_tokens(rendered: &str, tokens: &[&str]) {
        for token in tokens {
            assert!(
                !rendered.contains(token),
                "embodied render leaked debug token {token:?}:\n{rendered}"
            );
        }
    }
}
