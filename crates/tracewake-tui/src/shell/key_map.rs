use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::intent::{key_script::parse_key_token, UiIntent};

pub fn key_event_to_intent(event: KeyEvent) -> Option<UiIntent> {
    if !matches!(event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
        return None;
    }

    let token = key_event_token(event)?;
    parse_key_token(token).ok()
}

fn key_event_token(event: KeyEvent) -> Option<&'static str> {
    match event.code {
        KeyCode::Tab => Some("Tab"),
        KeyCode::Up => Some("Up"),
        KeyCode::Down => Some("Down"),
        KeyCode::Enter => Some("Enter"),
        KeyCode::Char('w') if plain_key(event.modifiers) => Some("w"),
        KeyCode::Char('c') if plain_key(event.modifiers) => Some("c"),
        KeyCode::Char('?') if plain_key(event.modifiers) => Some("?"),
        KeyCode::Char('n') if plain_key(event.modifiers) => Some("n"),
        KeyCode::Char('q') if plain_key(event.modifiers) => Some("q"),
        _ => None,
    }
}

fn plain_key(modifiers: KeyModifiers) -> bool {
    modifiers.is_empty() || modifiers == KeyModifiers::SHIFT
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::key_script::{parse_key_script, KeyScriptError};
    use crossterm::event::KeyEventState;

    #[test]
    fn crossterm_keys_match_script_intents_for_equivalent_keys() {
        let keys = [
            key(KeyCode::Tab),
            key(KeyCode::Down),
            key(KeyCode::Enter),
            char_key('w'),
            char_key('c'),
            char_key('?'),
            char_key('n'),
            char_key('q'),
        ];
        let shell_intents = keys
            .into_iter()
            .map(key_event_to_intent)
            .collect::<Option<Vec<_>>>()
            .expect("representative keys map to intents");

        let script_intents = parse_key_script("Tab Down Enter w c ? n q").unwrap();
        assert_eq!(shell_intents, script_intents);
    }

    #[test]
    fn release_and_unknown_keys_are_ignored() {
        assert_eq!(
            key_event_to_intent(KeyEvent::new_with_kind(
                KeyCode::Char('q'),
                KeyModifiers::NONE,
                KeyEventKind::Release,
            )),
            None
        );
        assert_eq!(key_event_to_intent(char_key('x')), None);
        assert_eq!(key_event_to_intent(ctrl_char_key('c')), None);
    }

    #[test]
    fn shared_parser_still_fails_closed_for_unknown_tokens() {
        assert_eq!(
            parse_key_token("Esc"),
            Err(KeyScriptError::UnknownToken {
                token: "Esc".to_string(),
            })
        );
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn char_key(character: char) -> KeyEvent {
        key(KeyCode::Char(character))
    }

    fn ctrl_char_key(character: char) -> KeyEvent {
        KeyEvent {
            code: KeyCode::Char(character),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }
}
