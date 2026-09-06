use crate::components::input;
use anathema::{
    component::Component,
    state::{List, State, Value},
};

const MESSAGE_FROM_USER: &str = "user";
const MESSAGE_FROM_AI: &str = "assistant";

pub struct HomePage;

impl Component for HomePage {
    type State = HomeState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        event.stop_propagation();

        if let Some(event) = event.data_checked::<input::Event>() {
            match event {
                input::Event::OnSubmit(value) => {
                    state.messages.push(value.to_owned());
                    state.messages_from.push(MESSAGE_FROM_USER.to_owned());
                }
                input::Event::OnUpdate(_) => (),
            }
        }
    }

    fn accept_focus(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    PromptSubmitted(String),
    None,
}

impl Event {
    pub fn name(&self) -> String {
        self.clone().into()
    }
}

impl From<Event> for String {
    fn from(value: Event) -> Self {
        match value {
            Event::PromptSubmitted(_) => "PromptSubmitted",
            Event::None => "None ",
        }
        .to_owned()
    }
}

#[derive(Debug, State, Default)]
pub struct HomeState {
    messages: Value<List<String>>,
    messages_from: Value<List<String>>,
}

impl HomeState {
    pub fn new() -> Self {
        Self::default()
    }
}
