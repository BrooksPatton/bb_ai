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
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.route.set(Route::Splash.to_string());
    }
}

#[derive(Debug, State, Default)]
pub struct AppState {
    route: Value<String>,
}
