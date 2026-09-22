use anathema::{
    component::{Component, KeyCode},
    state::{State, Value},
};

pub const NAME: &str = "bb_checkbox";

pub struct BBCheckbox;

impl Component for BBCheckbox {
    type State = BBCheckboxState;

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
        match key.code {
            KeyCode::Char(entered_char) => {
                if entered_char == ' ' {
                    let mut checked = state.checked.to_mut();

                    *checked = !*checked;

                    let event_data = if *checked {
                        BBCheckboxEvent::Checked
                    } else {
                        BBCheckboxEvent::Unchecked
                    };

                    context.publish("updated", event_data);
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug, State, Default)]
pub struct BBCheckboxState {
    focus: Value<String>,
    checked: Value<bool>,
}

pub enum BBCheckboxEvent {
    Checked,
    Unchecked,
}
