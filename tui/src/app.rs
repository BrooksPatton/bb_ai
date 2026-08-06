use crate::{pages::home, router::Route};
use anathema::{
    component::Component,
    geometry::Size,
    state::{State, Value},
};
use eyre::{Context, OptionExt, Result};

pub struct App;

#[derive(Debug, State)]
pub struct AppState {
    path: Value<String>,
    width: Value<u16>,
    height: Value<u16>,
    openrouter_key: Value<String>,
    route: Value<String>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let path = std::env::current_dir()
            .context("Setting the initial path on launch.")?
            .to_str()
            .ok_or_eyre("Converting path environment to string")?
            .to_owned();
        let width = Value::new(0);
        let height = Value::new(0);
        let openrouter_key = Value::new(String::new());
        let route = Route::ModelChooser.as_value();

        Ok(Self {
            path: Value::new(path),
            width,
            height,
            openrouter_key,
            route,
        })
    }
}

impl Component for App {
    type State = AppState;
    type Message = AppMessage;

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Ok(openrouter_key) = std::env::var("OPENROUTER_API_KEY") {
            state.openrouter_key.set(openrouter_key);
        }

        self.set_size(context, state);
    }

    fn on_resize(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        self.set_size(context, state);
    }

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            AppMessage::SlashModel => state.route.set(Route::ModelChooser.into()),
        }
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Some(home_event) = event.data_checked::<home::Event>() {
            match home_event {
                home::Event::PromptSubmitted(prompt) => {
                    if prompt == "/model" {
                        let new_route = Route::ModelChooser;

                        state.route.set(new_route.name());
                    }
                }
                home::Event::None => (),
            }
        }
    }
}

impl App {
    fn set_size(
        &self,
        context: anathema::component::Context<'_, '_, AppState>,
        state: &mut AppState,
    ) {
        let Size { width, height } = context.viewport.size();

        state.width.set(width);
        state.height.set(height);
    }
}

pub enum AppMessage {
    SlashModel,
}
