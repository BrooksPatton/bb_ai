use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::router::Route;

pub const NAME: &str = "app";

pub struct App;

impl Component for App {
    type State = AppState;

    type Message = ();

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let size = context.viewport.size();
        let width = size.width;
        let height = size.height;

        state.route.set(Route::Splash.to_string());
        state.width.set(width);
        state.height.set(height);
    }
}

#[derive(Debug, State, Default)]
pub struct AppState {
    route: Value<String>,
    width: Value<u16>,
    height: Value<u16>,
}
