use tracewake_core::debug_capability::DEBUG_NON_DIEGETIC_MARKER;
use tracewake_core::view_models::ViewMode;

use crate::screen::model::{EmbodiedScreenModel, FocusedPane, TerminalSize};
use crate::screen::text_dump::embodied_screen_pane_dumps;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenDump {
    pub mode: ViewMode,
    pub terminal_size: TerminalSize,
    pub focused_pane: FocusedPane,
    pub panes: Vec<ScreenPaneDump>,
    pub action_refs: Vec<String>,
    pub debug_marker_present: bool,
    pub view_model_id: String,
    pub holder_known_context_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenPaneDump {
    pub name: String,
    pub lines: Vec<String>,
}

pub fn build_embodied_screen_dump(screen: &EmbodiedScreenModel) -> ScreenDump {
    let panes = embodied_screen_pane_dumps(screen)
        .into_iter()
        .map(|pane| ScreenPaneDump {
            name: pane.name.to_string(),
            lines: pane.lines,
        })
        .collect::<Vec<_>>();
    let debug_marker_present = panes
        .iter()
        .flat_map(|pane| pane.lines.iter())
        .any(|line| line.contains(DEBUG_NON_DIEGETIC_MARKER));

    ScreenDump {
        mode: screen.metadata.mode,
        terminal_size: screen.options.terminal_size,
        focused_pane: screen.options.focused_pane,
        panes,
        action_refs: screen
            .actions
            .semantic_actions
            .iter()
            .map(|action| action.semantic_action_id.as_str().to_string())
            .collect(),
        debug_marker_present,
        view_model_id: screen.metadata.view_model_id.as_str().to_string(),
        holder_known_context_hash: screen.metadata.holder_known_context_hash.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::checksum::HolderKnownContextHash;
    use tracewake_core::ids::{
        ActionId, ActorId, ContainerId, DoorId, HolderKnownContextId, ItemId, PlaceId,
        SemanticActionId, ViewModelId,
    };
    use tracewake_core::time::SimTick;
    use tracewake_core::view_models::{
        EmbodiedViewModel, SemanticActionEntry, ViewMode, VisibleActor, VisibleContainer,
        VisibleDoor, VisibleExit, VisibleItem, VisibleItemSource,
    };

    use crate::screen::model::{
        build_embodied_screen_model, RenderOptions, TerminalSize, ThemeOptions,
    };
    use crate::screen::text_dump::render_embodied_screen_dump;

    #[test]
    fn build_embodied_screen_dump_populates_structured_fields() {
        let screen = fixture_screen();
        let dump = build_embodied_screen_dump(&screen);

        assert_eq!(dump.mode, ViewMode::Embodied);
        assert_eq!(dump.terminal_size, TerminalSize::new(100, 30));
        assert_eq!(dump.focused_pane, FocusedPane::Actions);
        assert_eq!(dump.view_model_id, "vm-test");
        assert_eq!(
            dump.holder_known_context_hash,
            screen.metadata.holder_known_context_hash
        );
        assert_eq!(dump.action_refs, vec!["semantic_wait".to_string()]);
        assert!(dump.panes.iter().any(|pane| pane.name == "place"));
        assert!(dump.panes.iter().any(|pane| pane.name == "actions"));
    }

    #[test]
    fn structured_pane_dumps_match_plain_text_pane_content() {
        let screen = fixture_screen();
        let plain = render_embodied_screen_dump(&screen);
        let structured = build_embodied_screen_dump(&screen);

        for pane in &structured.panes {
            let pane_block = format!("PANE {}\n{}", pane.name, pane.lines.join("\n"));
            assert!(
                plain.contains(&pane_block),
                "plain text dump is missing structured pane block {pane_block}"
            );
        }
    }

    #[test]
    fn structured_dump_carries_no_debug_marker_or_churny_holder_known_fields() {
        let dump = build_embodied_screen_dump(&fixture_screen());

        assert!(!dump.debug_marker_present);
        assert!(!dump
            .panes
            .iter()
            .flat_map(|pane| pane.lines.iter())
            .any(|line| line.contains(DEBUG_NON_DIEGETIC_MARKER)));

        let debug = format!("{dump:?}");
        assert!(!debug.contains("holder_known_context_frontier"));
        assert!(!debug.contains("holder_known_context_source_summary"));
    }

    #[test]
    fn build_embodied_screen_dump_is_deterministic() {
        let screen = fixture_screen();

        assert_eq!(
            build_embodied_screen_dump(&screen),
            build_embodied_screen_dump(&screen)
        );
    }

    fn fixture_screen() -> EmbodiedScreenModel {
        build_embodied_screen_model(
            &fixture_view(),
            RenderOptions {
                terminal_size: TerminalSize::new(100, 30),
                focused_pane: FocusedPane::Actions,
                theme: ThemeOptions::default(),
            },
        )
    }

    fn fixture_view() -> EmbodiedViewModel {
        EmbodiedViewModel::for_test(
            ViewModelId::new("vm-test").unwrap(),
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
            vec![VisibleActor {
                actor_id: ActorId::new("actor_tomas").unwrap(),
            }],
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
            HolderKnownContextId::new("hkc-test").unwrap(),
            HolderKnownContextHash::from_canonical_lines(&["fixture".to_string()]),
            3,
            "fixture-source".to_string(),
            None,
            None,
            false,
        )
    }
}
