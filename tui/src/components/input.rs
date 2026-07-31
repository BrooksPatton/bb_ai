use anathema::{
    component::Component,
    state::{State, Value},
};

const CURSOR: &str = "|";

pub struct Input {
    value: String,
    cursor_pos: usize,
}

impl Input {
    pub fn new() -> Self {
        let value = String::new();
        let cursor_pos = 0;

        Self { value, cursor_pos }
    }
}

#[derive(Debug, State)]
pub struct InputState {
    value: Value<String>,
}

impl InputState {
    pub fn new() -> Self {
        let value = Value::new(String::from(CURSOR));

        Self { value }
    }
}

impl Component for Input {
    type State = InputState;

    type Message = ();
}
