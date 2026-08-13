use std::ops::Sub;

use anathema::{
    component::Component,
    default_widgets::Overflow,
    state::{List, State, Value},
};

pub struct ModelChooserPage;

impl Component for ModelChooserPage {
    type State = ModelChooserPageState;

    type Message = ();

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let height = context.viewport.size().height - 2;
        let models_to_retrieve = usize::from(height * 2);

        state.loading.set(true);

        if let Some(openrouter_api_key) = context.attributes.get_as::<&str>("openrouter_key") {
            let models =
                openrouter_api::get_list_of_models(openrouter_api_key, models_to_retrieve, 0)
                    .unwrap();
            let model_names = models.iter().map(|model| model.name.clone());
            let model_ids = models.iter().map(|model| model.id.clone());
            let model_context = models.iter().map(|model| model.context_length);
            let model_input_price = models.iter().map(|model| model.input_price.clone());
            let model_output_price = models.iter().map(|model| model.output_price.clone());

            state.model_names.set(List::from_iter(model_names));
            state.model_ids.set(List::from_iter(model_ids));
            state.model_context.set(List::from_iter(model_context));
            state
                .model_input_price
                .set(List::from_iter(model_input_price));
            state
                .model_output_price
                .set(List::from_iter(model_output_price));
        }

        state.loading.set(false);
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            anathema::component::KeyCode::Char(_) => todo!(),
            anathema::component::KeyCode::Tab => todo!(),
            anathema::component::KeyCode::BackTab => todo!(),
            anathema::component::KeyCode::CtrlC => todo!(),
            anathema::component::KeyCode::Backspace => todo!(),
            anathema::component::KeyCode::Enter => todo!(),
            anathema::component::KeyCode::Left => todo!(),
            anathema::component::KeyCode::Right => todo!(),
            anathema::component::KeyCode::Up => {
                children.elements().by_tag("overflow").first(|el, _| {
                    let overflow = el.to::<Overflow>();
                    let offset = overflow.offset().y as usize;
                    let index = *state.selected_index.to_ref();

                    if index == 0 {
                        return;
                    }

                    if index == offset {
                        overflow.scroll_up_by(1);
                    }

                    state.selected_index.set(index - 1);
                });
            }
            anathema::component::KeyCode::Down => {
                children.elements().by_tag("overflow").first(|el, _| {
                    let overflow = el.to::<Overflow>();
                    let offset = overflow.offset().y as usize;
                    let height = usize::from(context.viewport.size().height);
                    let screen_offset = 5;
                    let index = *state.selected_index.to_mut();

                    if index == height + offset - screen_offset {
                        overflow.scroll_down_by(1)
                    }

                    state.selected_index.set(index + 1);
                });
            }
            anathema::component::KeyCode::Home => todo!(),
            anathema::component::KeyCode::End => todo!(),
            anathema::component::KeyCode::PageUp => todo!(),
            anathema::component::KeyCode::PageDown => todo!(),
            anathema::component::KeyCode::Delete => todo!(),
            anathema::component::KeyCode::Insert => todo!(),
            anathema::component::KeyCode::F(_) => todo!(),
            anathema::component::KeyCode::Null => todo!(),
            anathema::component::KeyCode::Esc => todo!(),
            anathema::component::KeyCode::CapsLock => todo!(),
            anathema::component::KeyCode::ScrollLock => todo!(),
            anathema::component::KeyCode::NumLock => todo!(),
            anathema::component::KeyCode::PrintScreen => todo!(),
            anathema::component::KeyCode::Pause => todo!(),
            anathema::component::KeyCode::Menu => todo!(),
            anathema::component::KeyCode::KeypadBegin => todo!(),
        }
    }
}

#[derive(Debug, State, Default)]
pub struct ModelChooserPageState {
    loading: Value<bool>,
    model_names: Value<List<String>>,
    model_ids: Value<List<String>>,
    model_context: Value<List<usize>>,
    model_input_price: Value<List<String>>,
    model_output_price: Value<List<String>>,
    selected_index: Value<usize>,
}

impl ModelChooserPageState {
    pub fn new() -> Self {
        Self::default()
    }
}
