use anathema::{
    component::Component,
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
        state.loading.set(true);

        if let Some(openrouter_api_key) = context.attributes.get_as::<&str>("openrouter_key") {
            let models = openrouter_api::get_list_of_models(openrouter_api_key, 10, 0).unwrap();
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
}

#[derive(Debug, State, Default)]
pub struct ModelChooserPageState {
    loading: Value<bool>,
    model_names: Value<List<String>>,
    model_ids: Value<List<String>>,
    model_context: Value<List<usize>>,
    model_input_price: Value<List<String>>,
    model_output_price: Value<List<String>>,
}

impl ModelChooserPageState {
    pub fn new() -> Self {
        Self::default()
    }
}
