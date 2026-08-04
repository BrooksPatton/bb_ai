use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::app::AppMessage;

const CURSOR: &str = "|";

pub struct Input {
    value: Vec<char>,
    cursor_pos: usize,
}

impl Input {
    pub fn new() -> Self {
        let value = vec![];
        let cursor_pos = 0;

        Self { value, cursor_pos }
    }
}

#[derive(Debug, State)]
pub struct InputState {
    value: Value<String>,
    focus: Value<String>,
}

impl InputState {
    pub fn new() -> Self {
        let value = Value::new(String::from(CURSOR));
        let focus = Value::new(String::from("not_focus"));

        Self { value, focus }
    }
}

impl Component for Input {
    type State = InputState;

    type Message = ();

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.focus.set(String::from("focus"));
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.focus.set(String::from("not_focus"));
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            anathema::component::KeyCode::Char(character) => {
                self.value.insert(self.cursor_pos, character);
                self.cursor_pos += 1;

                let mut value_with_cursor = self.value.clone();
                value_with_cursor.insert(self.cursor_pos, CURSOR.chars().nth(0).unwrap());
                let value = value_with_cursor.iter().collect();

                state.value.set(value);
            }
            anathema::component::KeyCode::Tab => todo!(),
            anathema::component::KeyCode::BackTab => todo!(),
            anathema::component::KeyCode::CtrlC => todo!(),
            anathema::component::KeyCode::Backspace => todo!(),
            anathema::component::KeyCode::Enter => {
                let value = self.value.iter().collect::<String>();

                if value == "/model" {
                    let message = AppMessage::SlashModel;

                    context.components.by_name("app").send(message);
                }

                self.value.clear();
                self.cursor_pos = 0;
                state.value.set(String::from(CURSOR));
            }
            anathema::component::KeyCode::Left => todo!(),
            anathema::component::KeyCode::Right => todo!(),
            anathema::component::KeyCode::Up => todo!(),
            anathema::component::KeyCode::Down => todo!(),
            anathema::component::KeyCode::Home => todo!(),
            anathema::component::KeyCode::End => todo!(),
            anathema::component::KeyCode::PageUp => todo!(),
            anathema::component::KeyCode::PageDown => todo!(),
            anathema::component::KeyCode::Delete => todo!(),
            anathema::component::KeyCode::Insert => todo!(),
            anathema::component::KeyCode::F(_) => todo!(),
            anathema::component::KeyCode::Null => todo!(),
            anathema::component::KeyCode::Esc => todo!(),
            anathema::component::KeyCode::CapsLock => todo!(),
            anathema::component::KeyCode::ScrollLock => todo!(),
            anathema::component::KeyCode::NumLock => todo!(),
            anathema::component::KeyCode::PrintScreen => todo!(),
            anathema::component::KeyCode::Pause => todo!(),
            anathema::component::KeyCode::Menu => todo!(),
            anathema::component::KeyCode::KeypadBegin => todo!(),
        }
    }
}
