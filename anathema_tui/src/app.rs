use anathema::{
    component::Component,
    state::{State, Value},
};
use anathema_components::bb_message::{self, BBMessageData};
use logger::BBLog;
use std::env;

use crate::router::Route;

pub const NAME: &str = "app";

pub struct App {
    logger: BBLog,
}

impl App {
    pub fn new() -> Self {
        let logger = BBLog::new().with_file_path("bb_ai.log");

        Self { logger }
    }
}

impl Component for App {
    type State = AppState;

    type Message = AppMessage;

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let size = context.viewport.size();
        let width = size.width;
        let height = size.height;
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd.to_string_lossy().to_string(),
            Err(error) => {
                self.logger.log(&error, true).ok();
                context
                    .components
                    .by_name(bb_message::NAME)
                    .send(BBMessageData::Error(error.to_string()));

                String::new()
            }
        };

        state.route.set(Route::default().to_string());
        state.width.set(width);
        state.height.set(height);
        state.model.set("mlx-community/Qwen3.8-27B-8bit".to_owned());
        state.cwd.set(cwd);
        state.ai_api_url.set("http://localhost:8080".to_owned());
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            anathema::component::KeyCode::Char(_) => todo!(),
            anathema::component::KeyCode::Tab => todo!(),
            anathema::component::KeyCode::BackTab => todo!(),
            anathema::component::KeyCode::CtrlC => todo!(),
            anathema::component::KeyCode::Backspace => todo!(),
            anathema::component::KeyCode::Enter => {
                state.route.set(Route::Home.to_string());
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

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_resize(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let size = context.viewport.size();
        let width = size.width;
        let height = size.height;
        state.width.set(width);
        state.height.set(height);
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            AppMessage::NavigateTo(route) => state.route.set(route.to_string()),
        }
    }
}

#[derive(Debug, State, Default)]
pub struct AppState {
    route: Value<String>,
    width: Value<u16>,
    height: Value<u16>,
    message: Value<String>,
    model: Value<String>,
    cwd: Value<String>,
    ai_api_url: Value<String>,
}

pub enum AppMessage {
    NavigateTo(Route),
}
