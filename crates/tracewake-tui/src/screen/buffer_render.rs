use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};

use crate::screen::model::EmbodiedScreenModel;
use crate::screen::pane_bindings::{
    render_pane_region_bindings, validate_non_vacuous_region, visible_region_lines,
    PaneRegionBinding,
};
use crate::screen::pane_layout::PaneRegion;

pub const MINIMUM_PANE_COLUMNS: u16 = 50;
pub const MINIMUM_PANE_ROWS: u16 = 12;

pub fn render_embodied_to_buffer(model: &EmbodiedScreenModel, area: Rect) -> Buffer {
    let mut buffer = Buffer::empty(area);
    if area.width == 0 || area.height == 0 {
        return buffer;
    }
    if area.width < MINIMUM_PANE_COLUMNS || area.height < MINIMUM_PANE_ROWS {
        buffer.set_stringn(
            area.x,
            area.y,
            format!(
                "Screen too small for embodied panes; resize to at least {}x{}.",
                MINIMUM_PANE_COLUMNS, MINIMUM_PANE_ROWS
            ),
            usize::from(area.width),
            Style::default(),
        );
        return buffer;
    }

    let bindings = render_pane_region_bindings(model);
    if area.width < 80 {
        render_single_column(&mut buffer, area, &bindings);
    } else {
        render_two_column(&mut buffer, area, &bindings);
    }
    buffer
}

pub fn buffer_to_plain_text(buffer: &Buffer) -> String {
    let area = buffer.area;
    let width = usize::from(area.width);
    let height = usize::from(area.height);
    (0..height)
        .map(|row| {
            let start = row * width;
            let end = start + width;
            buffer.content[start..end]
                .iter()
                .map(|cell| cell.symbol())
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_single_column(buffer: &mut Buffer, area: Rect, bindings: &[PaneRegionBinding]) {
    let priority_heights = [
        (PaneRegion::HeaderModeBar, 2),
        (PaneRegion::DetailsWhyNot, 3),
        (PaneRegion::ActionsAffordances, 6),
        (PaneRegion::SelfBodyRoutine, 7),
        (PaneRegion::InputHintsFooter, 1),
    ];
    let mut y = area.y;
    let mut remaining_rows = area.height;
    for (region, desired_height) in priority_heights {
        if remaining_rows == 0 {
            return;
        }
        let height = desired_height.min(remaining_rows);
        if let Some(binding) = binding(bindings, region) {
            render_region(buffer, Rect::new(area.x, y, area.width, height), binding);
        }
        y = y.saturating_add(height);
        remaining_rows = remaining_rows.saturating_sub(height);
    }

    for region in [
        PaneRegion::PlaceSituation,
        PaneRegion::CoPresentActors,
        PaneRegion::NotebookLeads,
        PaneRegion::RecentChanges,
    ] {
        if remaining_rows == 0 {
            break;
        }
        if let Some(binding) = binding(bindings, region) {
            render_region(buffer, Rect::new(area.x, y, area.width, 1), binding);
        }
        y = y.saturating_add(1);
        remaining_rows = remaining_rows.saturating_sub(1);
    }
}

fn render_two_column(buffer: &mut Buffer, area: Rect, bindings: &[PaneRegionBinding]) {
    let header_height = 2.min(area.height);
    let footer_height = if area.height > header_height { 1 } else { 0 };
    let body_y = area.y.saturating_add(header_height);
    let body_height = area.height.saturating_sub(header_height + footer_height);

    if let Some(header) = binding(bindings, PaneRegion::HeaderModeBar) {
        render_region(
            buffer,
            Rect::new(area.x, area.y, area.width, header_height),
            header,
        );
    }
    if footer_height > 0 {
        if let Some(footer) = binding(bindings, PaneRegion::InputHintsFooter) {
            render_region(
                buffer,
                Rect::new(
                    area.x,
                    area.bottom() - footer_height,
                    area.width,
                    footer_height,
                ),
                footer,
            );
        }
    }

    if body_height == 0 {
        return;
    }

    let gutter = 1;
    let left_width = area.width / 2;
    let right_width = area.width.saturating_sub(left_width + gutter);
    let left = Rect::new(area.x, body_y, left_width, body_height);
    let right = Rect::new(
        area.x + left_width + gutter,
        body_y,
        right_width,
        body_height,
    );

    render_column(
        buffer,
        left,
        bindings,
        &[
            PaneRegion::PlaceSituation,
            PaneRegion::CoPresentActors,
            PaneRegion::NotebookLeads,
            PaneRegion::RecentChanges,
        ],
    );
    render_column(
        buffer,
        right,
        bindings,
        &[
            PaneRegion::DetailsWhyNot,
            PaneRegion::ActionsAffordances,
            PaneRegion::SelfBodyRoutine,
        ],
    );
}

fn render_column(
    buffer: &mut Buffer,
    area: Rect,
    bindings: &[PaneRegionBinding],
    regions: &[PaneRegion],
) {
    let mut y = area.y;
    let mut remaining_rows = area.height;
    for (index, region) in regions.iter().enumerate() {
        if remaining_rows == 0 {
            break;
        }
        let remaining_regions = (regions.len() - index) as u16;
        let height = if remaining_regions == 1 {
            remaining_rows
        } else {
            remaining_rows / remaining_regions
        }
        .max(1);
        if let Some(binding) = binding(bindings, *region) {
            render_region(buffer, Rect::new(area.x, y, area.width, height), binding);
        }
        y = y.saturating_add(height);
        remaining_rows = remaining_rows.saturating_sub(height);
    }
}

fn render_region(buffer: &mut Buffer, area: Rect, binding: &PaneRegionBinding) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let title = format!("[{}]", binding.title);
    buffer.set_stringn(
        area.x,
        area.y,
        title,
        usize::from(area.width),
        Style::default().add_modifier(Modifier::BOLD),
    );

    let visible_lines = visible_region_lines(binding, usize::from(area.height.saturating_sub(1)))
        .unwrap_or_else(|error| vec![format!("... {}", error.reason)]);
    let _ = validate_non_vacuous_region(binding, &visible_lines);

    for (offset, line) in visible_lines.iter().enumerate() {
        buffer.set_stringn(
            area.x,
            area.y + 1 + offset as u16,
            line,
            usize::from(area.width),
            Style::default(),
        );
    }
}

fn binding(bindings: &[PaneRegionBinding], region: PaneRegion) -> Option<&PaneRegionBinding> {
    bindings.iter().find(|binding| binding.region == region)
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

    use crate::screen::model::{build_embodied_screen_model, RenderOptions, TerminalSize};

    #[test]
    fn buffer_render_is_deterministic_for_identical_input() {
        let screen = fixture_screen(100, 30);
        let area = Rect::new(0, 0, 100, 30);

        assert_eq!(
            render_embodied_to_buffer(&screen, area),
            render_embodied_to_buffer(&screen, area)
        );
    }

    #[test]
    fn buffer_render_draws_actor_safe_binding_content() {
        let screen = fixture_screen(100, 30);
        let text = buffer_to_plain_text(&render_embodied_to_buffer(
            &screen,
            Rect::new(0, 0, 100, 30),
        ));

        assert!(text.contains("[Header / Mode]"));
        assert!(text.contains("actor=actor_mara mode=Embodied focus=place"));
        assert!(text.contains("[Place / Situation]"));
        assert!(text.contains("Kitchen (kitchen)"));
        assert!(text.contains("[Actions / Affordances]"));
        assert!(text.contains("1. Wait [semantic_wait]"));
    }

    fn fixture_screen(columns: u16, rows: u16) -> EmbodiedScreenModel {
        build_embodied_screen_model(
            &fixture_view(),
            RenderOptions {
                terminal_size: TerminalSize::new(columns, rows),
                ..RenderOptions::default()
            },
        )
    }

    fn fixture_view() -> EmbodiedViewModel {
        EmbodiedViewModel::for_test(
            ViewModelId::new("vm-buffer").unwrap(),
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
            HolderKnownContextId::new("hkc-buffer").unwrap(),
            HolderKnownContextHash::from_canonical_lines(&["buffer".to_string()]),
            3,
            "fixture-source".to_string(),
            None,
            None,
            false,
        )
    }
}
