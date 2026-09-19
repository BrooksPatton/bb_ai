mod app;
mod components;
mod pages;
mod router;

use anathema::{
    backend::{Backend, tui::TuiBackend},
    runtime::Runtime,
    templates::Document,
};
use anathema_components::{
    bb_button::{self, BBButton, BBButtonState},
    bb_input::{self, BBInput, BBInputState},
    bb_message::{self, BBMessage, BBMessageState},
};
use eyre::Result;

use crate::{
    app::{App, AppState},
    components::{
        message::{self, Message},
        message_history::{self, MessageHistory, MessageHistoryState},
    },
    pages::{
        config::{self, ConfigPage, ConfigPageState},
        home::{self, HomePage},
    },
};

pub fn run() -> Result<()> {
    let doc = Document::new("@app");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()?;

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    builder.default::<()>("router", "templates/router.aml")?;
    builder.default::<()>("splash_page", "templates/pages/splash.aml")?;
    builder.default::<()>("app_bar", "templates/components/app_bar.aml")?;
    builder.default::<()>("info_column", "templates/components/info_column.aml")?;
    builder.default::<()>("model_info", "templates/components/model_info.aml")?;

    builder.component(
        app::NAME,
        "templates/app.aml",
        App::new(),
        AppState::default(),
    )?;
    builder.component(
        bb_message::NAME,
        "templates/bb_components/message.aml",
        BBMessage,
        BBMessageState::default(),
    )?;
    builder.component(
        bb_input::NAME,
        "templates/bb_components/input.aml",
        BBInput::default(),
        BBInputState::default(),
    )?;
    builder.component(home::NAME, "templates/pages/home.aml", HomePage, ())?;
    builder.component(
        config::NAME,
        "templates/pages/config.aml",
        ConfigPage,
        ConfigPageState::default(),
    )?;
    builder.component(
        message_history::NAME,
        "templates/components/message_history.aml",
        MessageHistory,
        MessageHistoryState::default(),
    )?;

    builder.prototype(
        message::NAME,
        "templates/components/message.aml",
        || Message,
        || (),
    )?;
    builder.prototype(
        bb_button::NAME,
        "templates/bb_components/button.aml",
        || BBButton,
        BBButtonState::default,
    )?;

    builder.finish(&mut backend, |runtime, backend| runtime.run(backend))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_canary_test() {
        assert_eq!(5, 5);
    }
}
