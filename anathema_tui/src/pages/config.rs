use anathema::{component::Component, state::State};
use anathema_components::bb_button::BBButtonEvent;

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
        if let Some(_event) = event.data_checked::<BBButtonEvent>() {
            context
                .components
                .by_name(app::NAME)
                .send(AppMessage::NavigateTo(crate::router::Route::Home));
        }
    }
}

#[derive(State, Default)]
pub struct ConfigPageState {}
