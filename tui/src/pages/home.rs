use crate::components::input;
use anathema::component::Component;

pub struct HomePage;

impl Component for HomePage {
    type State = ();

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        event.stop_propagation();

        if let Some(event) = event.data_checked::<input::Event>() {
            match event {
                input::Event::OnSubmit(value) => {
                    let event = Event::PromptSubmitted(value.to_owned());

                    context.publish(&event.name(), event);
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
