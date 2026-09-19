use anathema::{
    component::Component,
    state::{List, State, Value},
};

pub const NAME: &str = "message_history";

pub struct MessageHistory;

impl Component for MessageHistory {
    type State = MessageHistoryState;

    type Message = MessageHistoryMessage;

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            MessageHistoryMessage::UserPrompt(prompt) => {
                state.messages.push(prompt);
                state.roles.push("user".to_owned());
            }
        }
    }
}

#[derive(State, Default)]
pub struct MessageHistoryState {
    messages: Value<List<String>>,
    roles: Value<List<String>>,
}

pub enum MessageHistoryMessage {
    UserPrompt(String),
}
