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

        Ok(Self {
            path: Value::new(path),
            width,
            height,
        })
    }
}

impl Component for App {
    type State = AppState;

    type Message = ();

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
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
