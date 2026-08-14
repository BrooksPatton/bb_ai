use crate::components::input;
use anathema::{
    component::Component,
    default_widgets::Overflow,
    state::{List, State, Value},
};
use openrouter_api::OpenRouterModel;
use std::ops::Sub;

#[derive(Debug, Default)]
pub struct ModelChooserPage(Vec<OpenRouterModel>);

impl ModelChooserPage {
    pub fn set_models<'a>(&self, models: Vec<OpenRouterModel>, state: &mut ModelChooserPageState) {
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
}

impl Component for ModelChooserPage {
    type State = ModelChooserPageState;

    type Message = ();

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let models_to_retrieve = 1000;

        state.loading.set(true);

        if let Some(openrouter_api_key) = context.attributes.get_as::<&str>("openrouter_key") {
            let models =
                openrouter_api::get_list_of_models(openrouter_api_key, models_to_retrieve, 0)
                    .unwrap();

            self.0 = models.clone();
            self.set_models(models, state);
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
                    let screen_offset = 8;
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

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Some(data) = event.data_checked::<input::Event>() {
            match data {
                input::Event::OnSubmit(partial) => {
                    if partial.is_empty() {
                        println!("empty search");
                        self.set_models(self.0.clone(), state);
                        return;
                    }

                    let models = self
                        .0
                        .iter()
                        .cloned()
                        .filter(|model| model.name.to_lowercase().contains(&partial.to_lowercase()))
                        .collect();

                    self.set_models(models, state);
                }
                input::Event::OnUpdate(partial) => {
                    if partial.is_empty() {
                        println!("empty search");
                        self.set_models(self.0.clone(), state);
                        return;
                    }

                    println!("searching for: '{partial}'");
                    let models = self
                        .0
                        .iter()
                        .cloned()
                        .filter(|model| {
                            model
                                .name
                                .to_lowercase()
                                .contains(partial.to_lowercase().as_str())
                        })
                        .collect::<Vec<OpenRouterModel>>();

                    self.set_models(models, state);
                }
            }
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
    filter: Value<String>,
}

impl ModelChooserPageState {
    pub fn new() -> Self {
        Self::default()
    }
}
