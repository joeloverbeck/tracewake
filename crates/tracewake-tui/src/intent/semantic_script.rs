use crate::screen::model::FocusedPane;
use tracewake_core::ids::SemanticActionId;

use super::{Direction, UiIntent};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticScriptError {
    UnknownLine { line: String },
    UnknownPane { pane: String },
    MalformedField { line: String },
    BadSemanticActionId { value: String },
    BadMaxTicks { value: String },
    UnsupportedSelection { line: String },
}

pub fn parse_semantic_script(script: &str) -> Result<Vec<UiIntent>, SemanticScriptError> {
    script
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_semantic_line)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_semantic_line(line: &str) -> Result<UiIntent, SemanticScriptError> {
    if let Some(pane) = line.strip_prefix("focus ") {
        return pane_name_to_focused_pane(pane).map(UiIntent::FocusPane);
    }
    if let Some(selection) = line.strip_prefix("select ") {
        return parse_selection(line, selection);
    }
    if let Some(direction) = line.strip_prefix("move ") {
        return parse_direction(direction)
            .map(UiIntent::MoveSelection)
            .ok_or_else(|| SemanticScriptError::UnknownLine {
                line: line.to_string(),
            });
    }
    if let Some(field) = line.strip_prefix("submit ") {
        let value = parse_single_field(line, field, "semantic_action_id")?;
        return SemanticActionId::new(value.to_string())
            .map(UiIntent::SubmitSemanticAction)
            .map_err(|_| SemanticScriptError::BadSemanticActionId {
                value: value.to_string(),
            });
    }
    if line == "wait_one_tick" {
        return Ok(UiIntent::WaitOneTick);
    }
    if let Some(field) = line.strip_prefix("continue_until ") {
        let value = parse_single_field(line, field, "max_ticks")?;
        let max_ticks = value
            .parse::<u64>()
            .map_err(|_| SemanticScriptError::BadMaxTicks {
                value: value.to_string(),
            })?;
        return Ok(UiIntent::ContinueUntil { max_ticks });
    }
    match line {
        "help" => Ok(UiIntent::ToggleHelp),
        "notebook" => Ok(UiIntent::FocusNotebook),
        "quit" => Ok(UiIntent::Quit),
        _ => Err(SemanticScriptError::UnknownLine {
            line: line.to_string(),
        }),
    }
}

fn parse_selection(line: &str, selection: &str) -> Result<UiIntent, SemanticScriptError> {
    let parts = selection.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
        ["actor", "1"] | ["action", "1"] => Ok(UiIntent::ActivateSelection),
        ["actor", _] | ["action", _] => Err(SemanticScriptError::UnsupportedSelection {
            line: line.to_string(),
        }),
        _ => Err(SemanticScriptError::UnknownLine {
            line: line.to_string(),
        }),
    }
}

fn parse_direction(direction: &str) -> Option<Direction> {
    match direction {
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        _ => None,
    }
}

fn parse_single_field<'a>(
    line: &str,
    field: &'a str,
    expected_key: &str,
) -> Result<&'a str, SemanticScriptError> {
    let Some((key, value)) = field.split_once('=') else {
        return Err(SemanticScriptError::MalformedField {
            line: line.to_string(),
        });
    };
    if key != expected_key || value.is_empty() || value.contains('=') || value.contains(' ') {
        return Err(SemanticScriptError::MalformedField {
            line: line.to_string(),
        });
    }
    Ok(value)
}

fn pane_name_to_focused_pane(pane: &str) -> Result<FocusedPane, SemanticScriptError> {
    match pane {
        "place" => Ok(FocusedPane::Place),
        "exits" => Ok(FocusedPane::Exits),
        "doors" => Ok(FocusedPane::Doors),
        "containers" => Ok(FocusedPane::Containers),
        "items" => Ok(FocusedPane::Items),
        "inventory" => Ok(FocusedPane::Inventory),
        "actors" => Ok(FocusedPane::Actors),
        "actions" => Ok(FocusedPane::Actions),
        "status" => Ok(FocusedPane::Status),
        "why_not" => Ok(FocusedPane::WhyNot),
        "notebook" => Ok(FocusedPane::Notebook),
        "actor_known_interval" => Ok(FocusedPane::ActorKnownInterval),
        _ => Err(SemanticScriptError::UnknownPane {
            pane: pane.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representative_semantic_script_maps_to_exact_intent_stream() {
        let script = "\
focus actors
select actor 1
submit semantic_action_id=open.container.strongbox_tomas
wait_one_tick
continue_until max_ticks=4
";

        let parsed = parse_semantic_script(script).unwrap();

        assert_eq!(
            parsed,
            vec![
                UiIntent::FocusPane(FocusedPane::Actors),
                UiIntent::ActivateSelection,
                UiIntent::SubmitSemanticAction(
                    SemanticActionId::new("open.container.strongbox_tomas").unwrap()
                ),
                UiIntent::WaitOneTick,
                UiIntent::ContinueUntil { max_ticks: 4 },
            ]
        );
    }

    #[test]
    fn unknown_line_pane_and_malformed_field_fail_closed() {
        assert_eq!(
            parse_semantic_script("teleport here").unwrap_err(),
            SemanticScriptError::UnknownLine {
                line: "teleport here".to_string()
            }
        );
        assert_eq!(
            parse_semantic_script("focus truth").unwrap_err(),
            SemanticScriptError::UnknownPane {
                pane: "truth".to_string()
            }
        );
        assert_eq!(
            parse_semantic_script("continue_until ticks=4").unwrap_err(),
            SemanticScriptError::MalformedField {
                line: "continue_until ticks=4".to_string()
            }
        );
    }

    #[test]
    fn semantic_script_parse_is_deterministic() {
        let script = "\
focus actors
move down
select actor 1
submit semantic_action_id=open.container.strongbox_tomas
wait_one_tick
continue_until max_ticks=4
notebook
help
quit
";

        assert_eq!(
            parse_semantic_script(script).unwrap(),
            parse_semantic_script(script).unwrap()
        );
    }
}
