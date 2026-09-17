use std::time::Duration;

use anathema::{
    component::Component,
    state::{State, Value},
};

pub const NAME: &str = "bb_message";
pub const KEEP_FOR: u16 = 30 * 30;

pub struct BBMessage;

impl Component for BBMessage {
    type State = BBMessageState;

    type Message = BBMessageData;

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            BBMessageData::Error(msg) => {
                state.message.set(msg);
                state.theme.set("error".to_owned());
            }
            BBMessageData::Normal(msg) => {
                state.message.set(msg);
                state.theme.set("normal".to_owned());
            }
        }

        state.keep_for.set(KEEP_FOR);
    }

    fn on_tick(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        _context: anathema::component::Context<'_, '_, Self::State>,
        _dt: Duration,
    ) {
        let keep_for = state.keep_for.to_ref().saturating_sub(1);
        state.keep_for.set(keep_for);
    }
}

#[derive(Debug, State, Default)]
pub struct BBMessageState {
    message: Value<String>,
    theme: Value<String>,
    keep_for: Value<u16>,
}

pub enum BBMessageData {
    Error(String),
    Normal(String),
}
