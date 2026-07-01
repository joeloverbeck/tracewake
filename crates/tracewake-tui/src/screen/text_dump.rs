use tracewake_core::view_models::{VisibleItemSource, WhyNotView};

use crate::screen::model::{EmbodiedScreenModel, FocusedPane};

pub fn render_embodied_screen_dump(screen: &EmbodiedScreenModel) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "SCREEN mode={:?} size={}x{} focus={}",
        screen.metadata.mode,
        screen.options.terminal_size.columns,
        screen.options.terminal_size.rows,
        focused_pane_label(screen.options.focused_pane)
    ));
    lines.push(format!(
        "META view_model_id={} actor={} tick={} holder_known_hash={}",
        screen.metadata.view_model_id.as_str(),
        screen.metadata.viewer_actor_id.as_str(),
        screen.metadata.sim_tick.value(),
        screen.metadata.holder_known_context_hash
    ));

    for pane in embodied_screen_pane_dumps(screen) {
        lines.push(format!("PANE {}", pane.name));
        lines.extend(pane.lines);
    }

    lines.join("\n")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PaneDump {
    pub(crate) name: &'static str,
    pub(crate) lines: Vec<String>,
}

pub(crate) fn embodied_screen_pane_dumps(screen: &EmbodiedScreenModel) -> Vec<PaneDump> {
    vec![
        PaneDump {
            name: "place",
            lines: vec![format!(
                "{} ({})",
                screen.place.place_label,
                screen.place.place_id.as_str()
            )],
        },
        PaneDump {
            name: "exits",
            lines: list_or_none(screen.exits.visible_exits.iter().map(|exit| {
                let blocker = exit
                    .blocker_summary
                    .as_ref()
                    .map(|summary| format!(" blocked={summary}"))
                    .unwrap_or_default();
                format!("- {}{}", exit.destination_place_id.as_str(), blocker)
            })),
        },
        PaneDump {
            name: "doors",
            lines: list_or_none(screen.doors.visible_doors.iter().map(|door| {
                format!(
                    "- {} open={} locked={} endpoints={}<->{}",
                    door.door_id.as_str(),
                    door.is_open,
                    door.is_locked,
                    door.endpoint_a.as_str(),
                    door.endpoint_b.as_str()
                )
            })),
        },
        PaneDump {
            name: "containers",
            lines: list_or_none(screen.containers.visible_containers.iter().map(|container| {
                format!(
                    "- {} open={} locked={}",
                    container.container_id.as_str(),
                    container.is_open,
                    container.is_locked
                )
            })),
        },
        PaneDump {
            name: "items",
            lines: list_or_none(screen.items.visible_items.iter().map(render_item)),
        },
        PaneDump {
            name: "inventory",
            lines: list_or_none(screen.inventory.carried_items.iter().map(render_item)),
        },
        PaneDump {
            name: "actors",
            lines: list_or_none(
                screen
                    .actors
                    .local_actors
                    .iter()
                    .map(|actor| format!("- {}", actor.actor_id.as_str())),
            ),
        },
        PaneDump {
            name: "actions",
            lines: list_or_none(screen.actions.semantic_actions.iter().enumerate().map(
                |(index, action)| {
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
                            format!(" disabled={summary} reasons={reason_codes}")
                        })
                        .unwrap_or_default();
                    format!(
                        "{}. {} [{}]{}",
                        index + 1,
                        action.label,
                        action.semantic_action_id.as_str(),
                        disabled
                    )
                },
            )),
        },
        PaneDump {
            name: "status",
            lines: render_status(screen),
        },
        PaneDump {
            name: "why_not",
            lines: render_why_not(screen.why_not.last_rejection_why_not.as_ref(), screen),
        },
        PaneDump {
            name: "notebook",
            lines: render_notebook(screen),
        },
        PaneDump {
            name: "actor_known_interval",
            lines: render_actor_known_interval(screen),
        },
    ]
}

fn render_item(item: &tracewake_core::view_models::VisibleItem) -> String {
    format!(
        "- {} portable={} source={}",
        item.item_id.as_str(),
        item.portable,
        visible_item_source_label(&item.source)
    )
}

fn visible_item_source_label(source: &VisibleItemSource) -> String {
    match source {
        VisibleItemSource::Place => "place".to_string(),
        VisibleItemSource::Container(container_id) => format!("container:{}", container_id.as_str()),
        VisibleItemSource::Carried => "carried".to_string(),
    }
}

fn render_status(screen: &EmbodiedScreenModel) -> Vec<String> {
    let Some(status) = &screen.phase3a_status.status else {
        return vec!["- none".to_string()];
    };

    let mut lines = Vec::new();
    lines.push("Needs:".to_string());
    lines.extend(list_or_none(status.need_summaries.iter().map(|need| {
        format!(
            "- {} band={} cause={}",
            need.kind, need.band_label, need.last_cause
        )
    })));
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
    lines
}

fn render_why_not(why_not: Option<&WhyNotView>, screen: &EmbodiedScreenModel) -> Vec<String> {
    if let Some(why_not) = why_not {
        let mut lines = vec![format!(
            "{} kind={} reasons={}",
            why_not.actor_known_summary,
            why_not.failure_kind.stable_id(),
            why_not.reason_codes.join(",")
        )];
        if !why_not.actor_visible_facts.is_empty() {
            lines.push(format!(
                "facts={}",
                why_not.actor_visible_facts.join(",")
            ));
        }
        lines
    } else if let Some(summary) = &screen.why_not.last_rejection_summary {
        vec![summary.clone()]
    } else {
        vec!["- none".to_string()]
    }
}

fn render_notebook(screen: &EmbodiedScreenModel) -> Vec<String> {
    let Some(notebook) = &screen.notebook.notebook else {
        return vec!["- none".to_string()];
    };

    let mut lines = vec![format!("viewer={}", notebook.viewer_actor_id.as_str())];
    lines.push("Beliefs:".to_string());
    lines.extend(list_or_none(notebook.source_bound_beliefs.iter().map(|belief| {
        format!(
            "- {} confidence={} source={} tick={} :: {}",
            belief.belief_id,
            belief.confidence_label,
            belief.source_summary,
            belief.acquired_tick,
            belief.summary
        )
    })));
    lines.push("Observations:".to_string());
    lines.extend(list_or_none(notebook.recent_observations.iter().map(
        |observation| {
            format!(
                "- {} channel={} confidence={} tick={} :: {}",
                observation.observation_id,
                observation.channel,
                observation.confidence_label,
                observation.observed_tick,
                observation.summary
            )
        },
    )));
    lines.push("Contradictions:".to_string());
    lines.extend(
        list_or_none(notebook.known_contradictions.iter().map(|contradiction| {
            format!(
                "- {} :: {}",
                contradiction.contradiction_id, contradiction.summary
            )
        })),
    );
    lines.push("Leads:".to_string());
    lines.extend(list_or_none(notebook.typed_leads.iter().map(|lead| {
        format!(
            "- {} confidence={} staleness={} :: {}",
            lead.lead_id, lead.confidence_label, lead.staleness_label, lead.summary
        )
    })));
    if notebook.typed_leads.is_empty() {
        lines.extend(notebook.possible_leads.iter().map(|lead| format!("- {lead}")));
    }
    lines
}

fn render_actor_known_interval(screen: &EmbodiedScreenModel) -> Vec<String> {
    let Some(summary) = &screen.actor_known_interval.summary else {
        return vec!["- none".to_string()];
    };

    let mut lines = vec![format!(
        "no_new_actor_known_information={}",
        summary.no_new_actor_known_information()
    )];
    lines.extend(list_or_none(
        summary
            .notices()
            .iter()
            .map(|notice| format!("- {}", notice.notice_kind().stable_id())),
    ));
    lines
}

fn list_or_none<I>(items: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let lines = items.into_iter().collect::<Vec<_>>();
    if lines.is_empty() {
        vec!["- none".to_string()]
    } else {
        lines
    }
}

fn focused_pane_label(pane: FocusedPane) -> &'static str {
    match pane {
        FocusedPane::Place => "place",
        FocusedPane::Exits => "exits",
        FocusedPane::Doors => "doors",
        FocusedPane::Containers => "containers",
        FocusedPane::Items => "items",
        FocusedPane::Inventory => "inventory",
        FocusedPane::Actors => "actors",
        FocusedPane::Actions => "actions",
        FocusedPane::Status => "status",
        FocusedPane::WhyNot => "why_not",
        FocusedPane::Notebook => "notebook",
        FocusedPane::ActorKnownInterval => "actor_known_interval",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::debug_capability::DEBUG_NON_DIEGETIC_MARKER;
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

    #[test]
    fn render_embodied_screen_dump_emits_screen_and_panes() {
        let screen = fixture_screen();
        let dump = render_embodied_screen_dump(&screen);

        assert!(dump.starts_with("SCREEN mode=Embodied size=80x24 focus=place\n"));
        assert!(dump.contains("PANE place\nKitchen (kitchen)"));
        assert!(dump.contains("PANE exits\n- yard"));
        assert!(dump.contains("PANE doors\n- door_kitchen_yard open=true locked=false"));
        assert!(dump.contains("PANE items\n- apple portable=true source=place"));
        assert!(dump.contains("PANE inventory\n- ledger portable=true source=carried"));
        assert!(dump.contains("PANE actors\n- actor_tomas"));
        assert!(dump.contains("PANE actions\n1. Wait [semantic_wait]"));
        assert!(dump.contains("PANE why_not\nNo recent rejection"));
    }

    #[test]
    fn render_embodied_screen_dump_omits_debug_non_diegetic_marker() {
        let dump = render_embodied_screen_dump(&fixture_screen());

        assert!(!dump.contains(DEBUG_NON_DIEGETIC_MARKER));
    }

    #[test]
    fn render_embodied_screen_dump_is_deterministic() {
        let screen = fixture_screen();

        assert_eq!(
            render_embodied_screen_dump(&screen),
            render_embodied_screen_dump(&screen)
        );
    }

    fn fixture_screen() -> EmbodiedScreenModel {
        build_embodied_screen_model(
            &fixture_view(),
            RenderOptions {
                terminal_size: TerminalSize::new(80, 24),
                focused_pane: FocusedPane::Place,
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
