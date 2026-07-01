use super::{Direction, UiIntent};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyScriptError {
    UnknownToken { token: String },
}

pub fn parse_key_script(script: &str) -> Result<Vec<UiIntent>, KeyScriptError> {
    script
        .split_whitespace()
        .map(parse_key_token)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_key_token(token: &str) -> Result<UiIntent, KeyScriptError> {
    match token {
        "Tab" => Ok(UiIntent::FocusNext),
        "Up" => Ok(UiIntent::MoveSelection(Direction::Up)),
        "Down" => Ok(UiIntent::MoveSelection(Direction::Down)),
        "Enter" => Ok(UiIntent::ActivateSelection),
        "w" => Ok(UiIntent::WaitOneTick),
        "c" => Ok(UiIntent::ContinueUntil { max_ticks: 64 }),
        "?" => Ok(UiIntent::ToggleHelp),
        "n" => Ok(UiIntent::FocusNotebook),
        "q" => Ok(UiIntent::Quit),
        _ => Err(KeyScriptError::UnknownToken {
            token: token.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representative_key_script_maps_to_exact_intent_stream() {
        let parsed = parse_key_script("Tab Down Enter w c ? n q").unwrap();

        assert_eq!(
            parsed,
            vec![
                UiIntent::FocusNext,
                UiIntent::MoveSelection(Direction::Down),
                UiIntent::ActivateSelection,
                UiIntent::WaitOneTick,
                UiIntent::ContinueUntil { max_ticks: 64 },
                UiIntent::ToggleHelp,
                UiIntent::FocusNotebook,
                UiIntent::Quit,
            ]
        );
    }

    #[test]
    fn unknown_token_fails_closed() {
        let error = parse_key_script("Tab Hover").unwrap_err();
        assert_eq!(
            error,
            KeyScriptError::UnknownToken {
                token: "Hover".to_string()
            }
        );
    }

    #[test]
    fn key_script_parse_is_deterministic() {
        let script = "Tab Down Enter w c ? n q";
        assert_eq!(
            parse_key_script(script).unwrap(),
            parse_key_script(script).unwrap()
        );
    }
}
