use crate::screen::model::{
    ActionsPane, ActorKnownIntervalPane, ActorsPane, ContainersPane, DoorsPane,
    EmbodiedScreenModel, ExitsPane, FocusedPane, InventoryPane, ItemsPane, NotebookPane,
    Phase3AStatusPane, PlacePane, WhyNotPane,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PaneRegion {
    HeaderModeBar,
    PlaceSituation,
    SelfBodyRoutine,
    CoPresentActors,
    ActionsAffordances,
    DetailsWhyNot,
    NotebookLeads,
    RecentChanges,
    InputHintsFooter,
}

#[derive(Clone, Copy, Debug)]
pub enum ScreenPaneRef<'a> {
    Place(&'a PlacePane),
    Exits(&'a ExitsPane),
    Doors(&'a DoorsPane),
    Containers(&'a ContainersPane),
    Items(&'a ItemsPane),
    Inventory(&'a InventoryPane),
    Actors(&'a ActorsPane),
    Actions(&'a ActionsPane),
    Status(&'a Phase3AStatusPane),
    WhyNot(&'a WhyNotPane),
    Notebook(&'a NotebookPane),
    ActorKnownInterval(&'a ActorKnownIntervalPane),
}

impl ScreenPaneRef<'_> {
    pub const fn focused_pane(self) -> FocusedPane {
        match self {
            Self::Place(_) => FocusedPane::Place,
            Self::Exits(_) => FocusedPane::Exits,
            Self::Doors(_) => FocusedPane::Doors,
            Self::Containers(_) => FocusedPane::Containers,
            Self::Items(_) => FocusedPane::Items,
            Self::Inventory(_) => FocusedPane::Inventory,
            Self::Actors(_) => FocusedPane::Actors,
            Self::Actions(_) => FocusedPane::Actions,
            Self::Status(_) => FocusedPane::Status,
            Self::WhyNot(_) => FocusedPane::WhyNot,
            Self::Notebook(_) => FocusedPane::Notebook,
            Self::ActorKnownInterval(_) => FocusedPane::ActorKnownInterval,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PaneRegionLayout<'a> {
    pub region: PaneRegion,
    pub panes: Vec<ScreenPaneRef<'a>>,
}

pub const PANE_REGION_ORDER: [PaneRegion; 9] = [
    PaneRegion::HeaderModeBar,
    PaneRegion::DetailsWhyNot,
    PaneRegion::ActionsAffordances,
    PaneRegion::SelfBodyRoutine,
    PaneRegion::PlaceSituation,
    PaneRegion::CoPresentActors,
    PaneRegion::NotebookLeads,
    PaneRegion::RecentChanges,
    PaneRegion::InputHintsFooter,
];

pub const FOCUSED_PANE_ORDER: [FocusedPane; 12] = [
    FocusedPane::Place,
    FocusedPane::Exits,
    FocusedPane::Doors,
    FocusedPane::Containers,
    FocusedPane::Items,
    FocusedPane::Inventory,
    FocusedPane::Actors,
    FocusedPane::Actions,
    FocusedPane::Status,
    FocusedPane::WhyNot,
    FocusedPane::Notebook,
    FocusedPane::ActorKnownInterval,
];

pub fn pane_regions() -> &'static [PaneRegion; 9] {
    &PANE_REGION_ORDER
}

pub const fn region_for_focused_pane(focused_pane: FocusedPane) -> PaneRegion {
    match focused_pane {
        FocusedPane::Place
        | FocusedPane::Exits
        | FocusedPane::Doors
        | FocusedPane::Containers
        | FocusedPane::Items => PaneRegion::PlaceSituation,
        FocusedPane::Inventory | FocusedPane::Status => PaneRegion::SelfBodyRoutine,
        FocusedPane::Actors => PaneRegion::CoPresentActors,
        FocusedPane::Actions => PaneRegion::ActionsAffordances,
        FocusedPane::WhyNot => PaneRegion::DetailsWhyNot,
        FocusedPane::Notebook => PaneRegion::NotebookLeads,
        FocusedPane::ActorKnownInterval => PaneRegion::RecentChanges,
    }
}

pub fn embodied_pane_layout(model: &EmbodiedScreenModel) -> Vec<PaneRegionLayout<'_>> {
    pane_regions()
        .iter()
        .copied()
        .map(|region| PaneRegionLayout {
            region,
            panes: panes_for_region(model, region),
        })
        .collect()
}

pub fn panes_for_region(model: &EmbodiedScreenModel, region: PaneRegion) -> Vec<ScreenPaneRef<'_>> {
    match region {
        PaneRegion::HeaderModeBar | PaneRegion::InputHintsFooter => Vec::new(),
        PaneRegion::PlaceSituation => vec![
            ScreenPaneRef::Place(&model.place),
            ScreenPaneRef::Exits(&model.exits),
            ScreenPaneRef::Doors(&model.doors),
            ScreenPaneRef::Containers(&model.containers),
            ScreenPaneRef::Items(&model.items),
        ],
        PaneRegion::SelfBodyRoutine => vec![
            ScreenPaneRef::Status(&model.phase3a_status),
            ScreenPaneRef::Inventory(&model.inventory),
        ],
        PaneRegion::CoPresentActors => vec![ScreenPaneRef::Actors(&model.actors)],
        PaneRegion::ActionsAffordances => vec![ScreenPaneRef::Actions(&model.actions)],
        PaneRegion::DetailsWhyNot => vec![ScreenPaneRef::WhyNot(&model.why_not)],
        PaneRegion::NotebookLeads => vec![ScreenPaneRef::Notebook(&model.notebook)],
        PaneRegion::RecentChanges => {
            vec![ScreenPaneRef::ActorKnownInterval(
                &model.actor_known_interval,
            )]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};
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

    use crate::screen::model::{build_embodied_screen_model, RenderOptions};

    #[test]
    fn pane_layout_region_order_is_deterministic_and_priority_ordered() {
        let screen = fixture_screen();
        let first = layout_signature(&screen);
        let second = layout_signature(&screen);

        assert_eq!(first, second);
        assert_eq!(
            embodied_pane_layout(&screen)
                .iter()
                .map(|layout| layout.region)
                .collect::<Vec<_>>(),
            vec![
                PaneRegion::HeaderModeBar,
                PaneRegion::DetailsWhyNot,
                PaneRegion::ActionsAffordances,
                PaneRegion::SelfBodyRoutine,
                PaneRegion::PlaceSituation,
                PaneRegion::CoPresentActors,
                PaneRegion::NotebookLeads,
                PaneRegion::RecentChanges,
                PaneRegion::InputHintsFooter,
            ]
        );
    }

    #[test]
    fn every_screen_pane_is_claimed_by_exactly_one_region() {
        let screen = fixture_screen();
        let mut claims = BTreeMap::<String, Vec<PaneRegion>>::new();

        for layout in embodied_pane_layout(&screen) {
            for pane in layout.panes {
                claims
                    .entry(format!("{:?}", pane.focused_pane()))
                    .or_default()
                    .push(layout.region);
            }
        }

        for focused_pane in FOCUSED_PANE_ORDER {
            let key = format!("{focused_pane:?}");
            assert_eq!(
                claims.get(&key).map(Vec::as_slice),
                Some(&[region_for_focused_pane(focused_pane)][..]),
                "{focused_pane:?} should map to exactly one expected region"
            );
        }

        assert_eq!(claims.len(), FOCUSED_PANE_ORDER.len());
    }

    #[test]
    fn focused_pane_region_mapping_is_exhaustive_and_region_owned() {
        let mapped_regions = FOCUSED_PANE_ORDER
            .iter()
            .copied()
            .map(region_for_focused_pane)
            .collect::<BTreeSet<_>>();

        assert!(mapped_regions.contains(&PaneRegion::PlaceSituation));
        assert!(mapped_regions.contains(&PaneRegion::SelfBodyRoutine));
        assert!(mapped_regions.contains(&PaneRegion::CoPresentActors));
        assert!(mapped_regions.contains(&PaneRegion::ActionsAffordances));
        assert!(mapped_regions.contains(&PaneRegion::DetailsWhyNot));
        assert!(mapped_regions.contains(&PaneRegion::NotebookLeads));
        assert!(mapped_regions.contains(&PaneRegion::RecentChanges));
        assert!(!mapped_regions.contains(&PaneRegion::HeaderModeBar));
        assert!(!mapped_regions.contains(&PaneRegion::InputHintsFooter));
    }

    fn layout_signature(screen: &EmbodiedScreenModel) -> Vec<(PaneRegion, Vec<FocusedPane>)> {
        embodied_pane_layout(screen)
            .into_iter()
            .map(|layout| {
                (
                    layout.region,
                    layout
                        .panes
                        .into_iter()
                        .map(ScreenPaneRef::focused_pane)
                        .collect(),
                )
            })
            .collect()
    }

    fn fixture_screen() -> EmbodiedScreenModel {
        build_embodied_screen_model(&fixture_view(), RenderOptions::default())
    }

    fn fixture_view() -> EmbodiedViewModel {
        EmbodiedViewModel::for_test(
            ViewModelId::new("vm-layout").unwrap(),
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
            vec![VisibleActor::identity_only(
                ActorId::new("actor_tomas").unwrap(),
            )],
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
            HolderKnownContextId::new("hkc-layout").unwrap(),
            HolderKnownContextHash::from_canonical_lines(&["layout".to_string()]),
            3,
            "fixture-source".to_string(),
            None,
            None,
            false,
        )
    }
}
