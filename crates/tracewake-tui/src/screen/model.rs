use tracewake_core::ids::{ActorId, HolderKnownContextId, PlaceId, ViewModelId};
use tracewake_core::time::SimTick;
use tracewake_core::view_models::{
    EmbodiedViewModel, NotebookView, Phase3AEmbodiedStatus, SemanticActionEntry,
    TypedActorKnownIntervalSummary, ViewMode, VisibleActor, VisibleContainer, VisibleDoor,
    VisibleExit, VisibleItem, WhyNotView,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbodiedScreenModel {
    pub options: RenderOptions,
    pub metadata: ScreenMetadata,
    pub place: PlacePane,
    pub exits: ExitsPane,
    pub doors: DoorsPane,
    pub containers: ContainersPane,
    pub items: ItemsPane,
    pub inventory: InventoryPane,
    pub actors: ActorsPane,
    pub actions: ActionsPane,
    pub phase3a_status: Phase3AStatusPane,
    pub why_not: WhyNotPane,
    pub notebook: NotebookPane,
    pub actor_known_interval: ActorKnownIntervalPane,
    pub debug: DebugPaneDisposition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderOptions {
    pub terminal_size: TerminalSize,
    pub focused_pane: FocusedPane,
    pub theme: ThemeOptions,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            terminal_size: TerminalSize::default(),
            focused_pane: FocusedPane::Place,
            theme: ThemeOptions::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerminalSize {
    pub columns: u16,
    pub rows: u16,
}

impl TerminalSize {
    pub const fn new(columns: u16, rows: u16) -> Self {
        Self { columns, rows }
    }
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self {
            columns: 80,
            rows: 24,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusedPane {
    Place,
    Exits,
    Doors,
    Containers,
    Items,
    Inventory,
    Actors,
    Actions,
    Status,
    WhyNot,
    Notebook,
    ActorKnownInterval,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ThemeOptions {
    pub high_contrast: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenMetadata {
    pub view_model_id: ViewModelId,
    pub mode: ViewMode,
    pub viewer_actor_id: ActorId,
    pub sim_tick: SimTick,
    pub holder_known_context_id: HolderKnownContextId,
    pub holder_known_context_hash: String,
    pub holder_known_context_frontier: u64,
    pub holder_known_context_source_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlacePane {
    pub place_id: PlaceId,
    pub place_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExitsPane {
    pub visible_exits: Vec<VisibleExit>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoorsPane {
    pub visible_doors: Vec<VisibleDoor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainersPane {
    pub visible_containers: Vec<VisibleContainer>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemsPane {
    pub visible_items: Vec<VisibleItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryPane {
    pub carried_items: Vec<VisibleItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorsPane {
    pub local_actors: Vec<VisibleActor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionsPane {
    pub semantic_actions: Vec<SemanticActionEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phase3AStatusPane {
    pub status: Option<Phase3AEmbodiedStatus>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhyNotPane {
    pub last_rejection_summary: Option<String>,
    pub last_rejection_why_not: Option<WhyNotView>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotebookPane {
    pub notebook: Option<NotebookView>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorKnownIntervalPane {
    pub summary: Option<TypedActorKnownIntervalSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugPaneDisposition {
    pub debug_available: bool,
    pub embodied_dump_suppresses_debug_marker: bool,
}

pub fn build_embodied_screen_model(
    view: &EmbodiedViewModel,
    opts: RenderOptions,
) -> EmbodiedScreenModel {
    EmbodiedScreenModel {
        options: opts,
        metadata: ScreenMetadata {
            view_model_id: view.view_model_id().clone(),
            mode: view.mode(),
            viewer_actor_id: view.viewer_actor_id().clone(),
            sim_tick: view.sim_tick(),
            holder_known_context_id: view.holder_known_context_id().clone(),
            holder_known_context_hash: view.holder_known_context_hash().as_str().to_string(),
            holder_known_context_frontier: view.holder_known_context_frontier(),
            holder_known_context_source_summary: view
                .holder_known_context_source_summary()
                .to_string(),
        },
        place: PlacePane {
            place_id: view.place_id.clone(),
            place_label: view.place_label.clone(),
        },
        exits: ExitsPane {
            visible_exits: view.visible_exits.clone(),
        },
        doors: DoorsPane {
            visible_doors: view.visible_doors.clone(),
        },
        containers: ContainersPane {
            visible_containers: view.visible_containers.clone(),
        },
        items: ItemsPane {
            visible_items: view.visible_items.clone(),
        },
        inventory: InventoryPane {
            carried_items: view.carried_items.clone(),
        },
        actors: ActorsPane {
            local_actors: view.local_actors.clone(),
        },
        actions: ActionsPane {
            semantic_actions: view.semantic_actions.clone(),
        },
        phase3a_status: Phase3AStatusPane {
            status: view.phase3a_status.clone(),
        },
        why_not: WhyNotPane {
            last_rejection_summary: view.last_rejection_summary.clone(),
            last_rejection_why_not: view.last_rejection_why_not.clone(),
        },
        notebook: NotebookPane {
            notebook: view.notebook.clone(),
        },
        actor_known_interval: ActorKnownIntervalPane {
            summary: view.actor_known_interval_summary().cloned(),
        },
        debug: DebugPaneDisposition {
            debug_available: view.debug_available(),
            embodied_dump_suppresses_debug_marker: true,
        },
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

    #[test]
    fn build_embodied_screen_model_maps_view_model_content_to_panes() {
        let view = fixture_view();
        let screen = build_embodied_screen_model(&view, RenderOptions::default());

        assert_eq!(screen.metadata.view_model_id, *view.view_model_id());
        assert_eq!(screen.metadata.mode, view.mode());
        assert_eq!(screen.metadata.viewer_actor_id, *view.viewer_actor_id());
        assert_eq!(screen.metadata.sim_tick, view.sim_tick());
        assert_eq!(screen.place.place_id, view.place_id);
        assert_eq!(screen.place.place_label, view.place_label);
        assert_eq!(screen.exits.visible_exits, view.visible_exits);
        assert_eq!(screen.doors.visible_doors, view.visible_doors);
        assert_eq!(
            screen.containers.visible_containers,
            view.visible_containers
        );
        assert_eq!(screen.items.visible_items, view.visible_items);
        assert_eq!(screen.inventory.carried_items, view.carried_items);
        assert_eq!(screen.actors.local_actors, view.local_actors);
        assert_eq!(screen.actions.semantic_actions, view.semantic_actions);
        assert_eq!(screen.phase3a_status.status, view.phase3a_status);
        assert_eq!(
            screen.why_not.last_rejection_summary,
            view.last_rejection_summary
        );
        assert_eq!(
            screen.why_not.last_rejection_why_not,
            view.last_rejection_why_not
        );
        assert_eq!(screen.notebook.notebook, view.notebook);
        assert_eq!(
            screen.actor_known_interval.summary.as_ref(),
            view.actor_known_interval_summary()
        );
        assert_eq!(screen.debug.debug_available, view.debug_available());
    }

    #[test]
    fn build_embodied_screen_model_is_deterministic_for_identical_input() {
        let view = fixture_view();
        let opts = RenderOptions {
            terminal_size: TerminalSize::new(100, 30),
            focused_pane: FocusedPane::Actions,
            theme: ThemeOptions {
                high_contrast: true,
            },
        };

        assert_eq!(
            build_embodied_screen_model(&view, opts.clone()),
            build_embodied_screen_model(&view, opts)
        );
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
