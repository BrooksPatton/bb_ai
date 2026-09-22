use anathema::{
    component::Component,
    state::{State, Value},
};
use anathema_components::{bb_button::BBButtonEvent, bb_checkbox::BBCheckboxEvent};

use crate::app::{self, AppMessage};

pub const NAME: &str = "config_page";

pub struct ConfigPage;

impl Component for ConfigPage {
    type State = ConfigPageState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "handle_cancel" => {
                context
                    .components
                    .by_name(app::NAME)
                    .send(AppMessage::NavigateTo(crate::router::Route::Home));
            }
            "handle_check_hugginface_connection" => {
                //
            }
            "handle_huggingface_active" => match event.data::<BBCheckboxEvent>() {
                BBCheckboxEvent::Checked => {
                    state.huggingface_active.set(true);
                    state.dirty.set(true);
                    context
                        .components
                        .by_name(app::NAME)
                        .send(AppMessage::LogError("activating huggingface".to_owned()));
                }
                BBCheckboxEvent::Unchecked => {
                    state.huggingface_active.set(false);
                    state.dirty.set(true);
                    context
                        .components
                        .by_name(app::NAME)
                        .send(AppMessage::LogError("de-activating huggingface".to_owned()));
                }
            },
            _ => (),
        }
    }
}

#[derive(State, Default)]
pub struct ConfigPageState {
    huggingface_active: Value<bool>,
    dirty: Value<bool>,
}
