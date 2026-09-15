use anathema::{
    component::Component,
    state::{State, Value},
};

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

                let mut value = self.value.clone();

                if let Some(use_cursor) = context
                    .attribute("cursor")
                    .map(|attr| attr.as_bool())
                    .flatten()
                {
                    if use_cursor {
                        value.insert(self.cursor_pos, CURSOR.chars().nth(0).unwrap());
                    }
                }

                let value: String = value.iter().collect();
                state.value.set(value.clone());

                let event = Event::OnUpdate(value);
                context.publish(&event.name(), event);
            }
            anathema::component::KeyCode::Tab => (),
            anathema::component::KeyCode::BackTab => (),
            anathema::component::KeyCode::CtrlC => (),
            anathema::component::KeyCode::Backspace => {
                if self.value.is_empty() {
                    return;
                }

                self.value.pop();
                self.cursor_pos -= 1;

                let mut value = self.value.clone();

                if let Some(use_cursor) = context
                    .attribute("cursor")
                    .map(|attr| attr.as_bool())
                    .flatten()
                {
                    if use_cursor {
                        value.insert(self.cursor_pos, CURSOR.chars().nth(0).unwrap());
                    }
                }

                let value: String = value.iter().collect();
                state.value.set(value.clone());

                let event = Event::OnUpdate(value);
                context.publish(&event.name(), event);
            }
            anathema::component::KeyCode::Enter => {
                let value = self.value.iter().collect::<String>();
                let event = Event::OnSubmit(value);

                self.value.clear();
                self.cursor_pos = 0;
                state.value.set(String::from(CURSOR));
                context.publish(&event.name(), event);
            }
            anathema::component::KeyCode::Left => (),
            anathema::component::KeyCode::Right => (),
            anathema::component::KeyCode::Up => (),
            anathema::component::KeyCode::Down => (),
            anathema::component::KeyCode::Home => (),
            anathema::component::KeyCode::End => (),
            anathema::component::KeyCode::PageUp => (),
            anathema::component::KeyCode::PageDown => (),
            anathema::component::KeyCode::Delete => (),
            anathema::component::KeyCode::Insert => (),
            anathema::component::KeyCode::F(_) => (),
            anathema::component::KeyCode::Null => (),
            anathema::component::KeyCode::Esc => (),
            anathema::component::KeyCode::CapsLock => (),
            anathema::component::KeyCode::ScrollLock => (),
            anathema::component::KeyCode::NumLock => (),
            anathema::component::KeyCode::PrintScreen => (),
            anathema::component::KeyCode::Pause => (),
            anathema::component::KeyCode::Menu => (),
            anathema::component::KeyCode::KeypadBegin => (),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    OnSubmit(String),
    OnUpdate(String),
}

impl Event {
    pub fn name(&self) -> String {
        self.to_owned().into()
    }
}

impl From<Event> for String {
    fn from(value: Event) -> Self {
        match value {
            Event::OnSubmit(_) => "OnSubmit",
            Event::OnUpdate(_) => "OnUpdate",
        }
        .to_owned()
    }
}
