use anathema::{component::Component, state::State};

pub struct Input;

#[derive(Debug, State)]
pub struct InputState {}

impl InputState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Input {
    type State = InputState;

    type Message = ();
}
