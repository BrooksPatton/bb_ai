use std::fmt::Display;

use anathema::{
    component::{Component, KeyCode},
    state::{List, State, Value},
};

pub const NAME: &str = "bb_input";

#[derive(Default)]
pub struct BBInput {
    value: String,
    cursor_position: usize,
}

impl Display for BBInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value = self.value.clone();
        value.insert(self.cursor_position, '|');

        write!(f, "{value}")
    }
}

impl Component for BBInput {
    type State = BBInputState;

    type Message = ();

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            KeyCode::Char(input_char) => {
                self.value.insert(self.cursor_position, input_char);
                self.cursor_position += 1;
                state.value.set(self.to_string());

                let event = BBInputEvent::Update(self.value.clone());
                context.publish(&event.to_string(), event);
            }
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::CtrlC => todo!(),
            KeyCode::Backspace => {
                self.value.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
                state.value.set(self.to_string());

                let event = BBInputEvent::Update(self.value.clone());
                context.publish(&event.to_string(), event);
            }
            KeyCode::Enter => {
                let event = BBInputEvent::Submitted(self.value.clone());
                context.publish(&event.to_string(), event);
                self.value = String::new();
                self.cursor_position = 0;
                state.value.set(self.to_string());
            }
            KeyCode::Left => {
                self.cursor_position -= 1;
                state.value.set(self.to_string());
            }
            KeyCode::Right => {
                self.cursor_position += 1;
                state.value.set(self.to_string());
            }
            KeyCode::Up | KeyCode::Home => {
                self.cursor_position = 0;
                state.value.set(self.to_string());
            }
            KeyCode::Down | KeyCode::End => {
                self.cursor_position = self.value.len();
                state.value.set(self.to_string());
            }
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
            KeyCode::CapsLock => todo!(),
            KeyCode::ScrollLock => todo!(),
            KeyCode::NumLock => todo!(),
            KeyCode::PrintScreen => todo!(),
            KeyCode::Pause => todo!(),
            KeyCode::Menu => todo!(),
            KeyCode::KeypadBegin => todo!(),
        }
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.focus.set("focus".to_owned());
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.focus.set("nofocus".to_owned());
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.value.set("|".to_owned());
    }
}

#[derive(Debug, State, Default)]
pub struct BBInputState {
    value: Value<String>,
    focus: Value<String>,
}

pub enum BBInputEvent {
    Update(String),
    Submitted(String),
}

impl Display for BBInputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let event = match self {
            BBInputEvent::Update(_) => "Update",
            BBInputEvent::Submitted(_) => "Submit",
        };

        write!(f, "{event}")
    }
}
