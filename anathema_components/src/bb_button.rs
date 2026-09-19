use std::fmt::Display;

use anathema::{
    component::{Component, KeyCode},
    state::{State, Value},
};

pub const NAME: &str = "bb_button";

pub struct BBButton;

impl Component for BBButton {
    type State = BBButtonState;

    type Message = ();

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

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let KeyCode::Enter = key.code {
            context.publish(&BBButtonEvent.to_string(), BBButtonEvent);
        }
    }
}

#[derive(State, Default)]
pub struct BBButtonState {
    focus: Value<String>,
}

pub struct BBButtonEvent;

impl Display for BBButtonEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "clicked")
    }
}
