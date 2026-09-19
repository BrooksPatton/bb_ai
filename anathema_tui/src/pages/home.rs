use anathema::component::Component;
use anathema_components::{
    bb_input::BBInputEvent,
    bb_message::{self, BBMessage, BBMessageData},
};

use crate::{
    app::{self, AppMessage},
    components::message_history::{self, MessageHistoryMessage},
};

pub const NAME: &str = "home_page";

pub struct HomePage;

impl Component for HomePage {
    type State = ();

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        event.stop_propagation();

        if let Some(input_event) = event.data_checked::<BBInputEvent>() {
            match input_event {
                BBInputEvent::Update(_) => todo!(),
                BBInputEvent::Submitted(user_prompt) => {
                    match user_prompt.to_lowercase().as_str() {
                        "/config" => context
                            .components
                            .by_name(app::NAME)
                            .send(AppMessage::NavigateTo(crate::router::Route::Config)),
                        _ => (),
                    }
                    context
                        .components
                        .by_name(message_history::NAME)
                        .send(MessageHistoryMessage::UserPrompt(user_prompt.to_owned()));
                }
            }
        }
    }
}
