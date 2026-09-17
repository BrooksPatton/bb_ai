use anathema::{
    component::Component,
    state::{State, Value},
};
use anathema_components::bb_message::{self, BBMessageData};

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

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            anathema::component::KeyCode::Char(_) => todo!(),
            anathema::component::KeyCode::Tab => todo!(),
            anathema::component::KeyCode::BackTab => todo!(),
            anathema::component::KeyCode::CtrlC => todo!(),
            anathema::component::KeyCode::Backspace => todo!(),
            anathema::component::KeyCode::Enter => {
                let message = BBMessageData::Error("This is a simulated error message".to_owned());
                context.components.by_name(bb_message::NAME).send(message);
            }
            anathema::component::KeyCode::Left => todo!(),
            anathema::component::KeyCode::Right => todo!(),
            anathema::component::KeyCode::Up => todo!(),
            anathema::component::KeyCode::Down => todo!(),
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
pub struct AppState {
    route: Value<String>,
    width: Value<u16>,
    height: Value<u16>,
    message: Value<String>,
}
