use crate::screen::model::EmbodiedScreenModel;
use crate::screen::pane_layout::{
    embodied_pane_layout, PaneRegion, PaneRegionLayout, ScreenPaneRef,
};
use crate::screen::text_dump::{embodied_screen_pane_dumps, focused_pane_label};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaneRegionBinding {
    pub region: PaneRegion,
    pub title: &'static str,
    pub lines: Vec<String>,
}

pub fn render_pane_region_bindings(screen: &EmbodiedScreenModel) -> Vec<PaneRegionBinding> {
    let pane_dumps = embodied_screen_pane_dumps(screen);
    embodied_pane_layout(screen)
        .into_iter()
        .map(|layout| render_region_binding(screen, &pane_dumps, layout))
        .collect()
}

fn render_region_binding(
    screen: &EmbodiedScreenModel,
    pane_dumps: &[crate::screen::text_dump::PaneDump],
    layout: PaneRegionLayout<'_>,
) -> PaneRegionBinding {
    let lines = match layout.region {
        PaneRegion::HeaderModeBar => render_header_lines(screen),
        PaneRegion::InputHintsFooter => render_input_hint_lines(),
        _ => render_grouped_pane_lines(pane_dumps, &layout.panes),
    };

    PaneRegionBinding {
        region: layout.region,
        title: region_title(layout.region),
        lines,
    }
}

fn render_header_lines(screen: &EmbodiedScreenModel) -> Vec<String> {
    vec![format!(
        "actor={} mode={:?} focus={} debug_available={}",
        screen.metadata.viewer_actor_id.as_str(),
        screen.metadata.mode,
        focused_pane_label(screen.options.focused_pane),
        screen.debug.debug_available
    )]
}

fn render_input_hint_lines() -> Vec<String> {
    vec!["focus panes; inspect focused; open details".to_string()]
}

fn render_grouped_pane_lines(
    pane_dumps: &[crate::screen::text_dump::PaneDump],
    panes: &[ScreenPaneRef<'_>],
) -> Vec<String> {
    let mut lines = Vec::new();
    for pane in panes {
        let pane_name = focused_pane_label(pane.focused_pane());
        lines.push(format!("{pane_name}:"));
        if let Some(pane_dump) = pane_dumps.iter().find(|dump| dump.name == pane_name) {
            lines.extend(pane_dump.lines.iter().cloned());
        } else {
            lines.push("- missing pane binding".to_string());
        }
    }
    lines
}

fn region_title(region: PaneRegion) -> &'static str {
    match region {
        PaneRegion::HeaderModeBar => "Header / Mode",
        PaneRegion::PlaceSituation => "Place / Situation",
        PaneRegion::SelfBodyRoutine => "Self / Body / Routine",
        PaneRegion::CoPresentActors => "Co-present Actors",
        PaneRegion::ActionsAffordances => "Actions / Affordances",
        PaneRegion::DetailsWhyNot => "Details / Why-not",
        PaneRegion::NotebookLeads => "Notebook / Leads",
        PaneRegion::RecentChanges => "Recent Actor-known Changes",
        PaneRegion::InputHintsFooter => "Input Hints",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::checksum::HolderKnownContextHash;
    use tracewake_core::debug_capability::DEBUG_NON_DIEGETIC_MARKER;
    use tracewake_core::ids::{
        ActionId, ActorId, ContainerId, DoorId, HolderKnownContextId, ItemId, PlaceId,
        SemanticActionId, ViewModelId,
    };
    use tracewake_core::time::SimTick;
    use tracewake_core::view_models::{
        ActorKnownActivitySourceKind, EmbodiedViewModel, ObservedActivityView,
        ObservedActorActivityKind, SemanticActionEntry, ViewMode, VisibleActor, VisibleContainer,
        VisibleDoor, VisibleExit, VisibleItem, VisibleItemSource,
    };

    use crate::screen::model::{build_embodied_screen_model, FocusedPane, RenderOptions};
    use crate::screen::text_dump::render_activity_disposition;

    #[test]
    fn render_pane_region_bindings_emits_actor_safe_region_lines() {
        let screen = fixture_screen(true);
        let bindings = render_pane_region_bindings(&screen);

        assert_eq!(
            bindings
                .iter()
                .map(|binding| binding.region)
                .collect::<Vec<_>>(),
            crate::screen::pane_layout::PANE_REGION_ORDER.to_vec()
        );

        let actors = binding_for(&bindings, PaneRegion::CoPresentActors);
        assert!(actors.lines.iter().any(|line| line.contains(
            "activity=working activity_summary=working at bench activity_source=direct perception"
        )));
        assert!(actors.lines.iter().any(|line| line == "- actor_ina"));

        let actions = binding_for(&bindings, PaneRegion::ActionsAffordances);
        assert!(actions
            .lines
            .iter()
            .any(|line| { line.contains("1. Wait [semantic_wait]") }));

        let place = binding_for(&bindings, PaneRegion::PlaceSituation);
        assert!(place.lines.iter().any(|line| line == "place:"));
        assert!(place
            .lines
            .iter()
            .any(|line| line.contains("Kitchen (kitchen)")));
    }

    #[test]
    fn header_binding_uses_debug_availability_without_debug_truth() {
        let mut screen = fixture_screen(true);
        screen.debug.debug_available = true;
        screen.options.focused_pane = FocusedPane::Actions;

        let bindings = render_pane_region_bindings(&screen);
        let header = binding_for(&bindings, PaneRegion::HeaderModeBar);

        assert_eq!(
            header.lines,
            vec!["actor=actor_mara mode=Embodied focus=actions debug_available=true"]
        );
        assert!(!header.lines.join("\n").contains(DEBUG_NON_DIEGETIC_MARKER));
        assert!(!header.lines.join("\n").contains("debug_only"));
    }

    #[test]
    fn embodied_region_lines_carry_no_non_diegetic_debug_marker() {
        let lines = render_pane_region_bindings(&fixture_screen(true))
            .into_iter()
            .flat_map(|binding| binding.lines)
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!lines.contains(DEBUG_NON_DIEGETIC_MARKER));
        assert!(!lines.contains("debug_only"));
    }

    #[test]
    fn region_bindings_are_deterministic() {
        let screen = fixture_screen(true);

        assert_eq!(
            render_pane_region_bindings(&screen),
            render_pane_region_bindings(&screen)
        );
    }

    #[test]
    fn binding_renderer_reuses_activity_disposition_format() {
        let actor = fixture_observed_actor();
        let expected = render_activity_disposition(actor.observed_activity.as_ref().unwrap());
        let screen = fixture_screen(true);
        let bindings = render_pane_region_bindings(&screen);
        let actors = binding_for(&bindings, PaneRegion::CoPresentActors);

        assert!(actors.lines.iter().any(|line| line.contains(&expected)));
    }

    fn binding_for(bindings: &[PaneRegionBinding], region: PaneRegion) -> &PaneRegionBinding {
        bindings
            .iter()
            .find(|binding| binding.region == region)
            .unwrap()
    }

    fn fixture_screen(with_observed_actor: bool) -> EmbodiedScreenModel {
        build_embodied_screen_model(&fixture_view(with_observed_actor), RenderOptions::default())
    }

    fn fixture_view(with_observed_actor: bool) -> EmbodiedViewModel {
        let mut actors = vec![VisibleActor::identity_only(
            ActorId::new("actor_ina").unwrap(),
        )];
        if with_observed_actor {
            actors.insert(0, fixture_observed_actor());
        }

        EmbodiedViewModel::for_test(
            ViewModelId::new("vm-bindings").unwrap(),
            ViewMode::Embodied,
            ActorId::new("actor_mara").unwrap(),
            SimTick::new(7),
            PlaceId::new("kitchen").unwrap(),
            "Kitchen".to_string(),
            vec![VisibleExit {
                destination_place_id: PlaceId::new("yard").unwrap(),
                blocker_summary: None,
            }],
            vec![VisibleDoor {
                door_id: DoorId::new("door_kitchen_yard").unwrap(),
                endpoint_a: PlaceId::new("kitchen").unwrap(),
                endpoint_b: PlaceId::new("yard").unwrap(),
                is_open: true,
                is_locked: false,
            }],
            vec![VisibleContainer {
                container_id: ContainerId::new("pantry").unwrap(),
                is_open: false,
                is_locked: false,
            }],
            vec![VisibleItem {
                item_id: ItemId::new("apple").unwrap(),
                source: VisibleItemSource::Place,
                portable: true,
            }],
            vec![VisibleItem {
                item_id: ItemId::new("ledger").unwrap(),
                source: VisibleItemSource::Carried,
                portable: true,
            }],
            actors,
            vec![SemanticActionEntry::new(
                SemanticActionId::new("semantic_wait").unwrap(),
                ActionId::new("wait").unwrap(),
                Vec::new(),
                "Wait",
                true,
                None,
            )],
            None,
            Some("No recent rejection".to_string()),
            None,
            HolderKnownContextId::new("hkc-bindings").unwrap(),
            HolderKnownContextHash::from_canonical_lines(&["bindings".to_string()]),
            3,
            "fixture-source".to_string(),
            None,
            None,
            false,
        )
    }

    fn fixture_observed_actor() -> VisibleActor {
        VisibleActor {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            display_label: "Tomas".to_string(),
            presence_source_summary: "event:event_visible_actor_tomas".to_string(),
            presence_observed_tick: SimTick::new(7),
            presence_staleness_label: "current".to_string(),
            presence_uncertainty_label: None,
            observed_activity: Some(ObservedActivityView {
                kind: ObservedActorActivityKind::Working,
                actor_safe_summary: "working at bench".to_string(),
                source: ActorKnownActivitySourceKind::DirectPerception,
                source_summary: "event:event_visible_actor_tomas".to_string(),
                observed_tick: SimTick::new(7),
                staleness_label: "current".to_string(),
                uncertainty_label: None,
            }),
        }
    }
}
