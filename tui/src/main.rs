use std::thread::spawn;

use agent::BBAgent;
// src/main.rs
use anathema::prelude::{Backend, Document, TuiBackend};
use anathema::runtime::Runtime;
use eyre::Result;
use tui::app::{self, App, AppState};
use tui::components::ai_message::AiMessage;
use tui::components::input::{Input, InputState};
use tui::pages::home::{self, HomePage, HomeState};
use tui::pages::model_chooser::{ModelChooserPage, ModelChooserPageState};

fn main() -> Result<()> {
    let doc = Document::new("@app");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();
    let mut builder = Runtime::builder(doc, &backend);
    let mut agent = BBAgent::new();
    let query_tx = agent.take_query_tx();
    let response_rx = agent.take_response_tx().unwrap();
    let agent_handle = spawn(move || {
        agent.run();
    });

    builder.default::<()>("connection_side", "templates/connections_side.aml")?;
    builder.default::<()>("model_modal", "templates/model_modal.aml")?;
    builder.default::<()>("router", "templates/router.aml")?;
    builder.default::<()>("top_nav", "templates/components/top_nav.aml")?;

    builder.component(
        app::NAME,
        "templates/index.aml",
        App(query_tx, response_rx),
        AppState::new()?,
    )?;
    builder.component(
        home::NAME,
        "templates/pages/home.aml",
        HomePage,
        HomeState::new(),
    )?;
    builder.component(
        "model_chooser",
        "templates/pages/model_chooser.aml",
        ModelChooserPage::default(),
        ModelChooserPageState::new(),
    )?;

    builder.prototype("input", "templates/input.aml", Input::new, InputState::new)?;
    builder.prototype(
        "ai_message",
        "templates/components/message.aml",
        || AiMessage,
        || (),
    )?;

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();

    Ok(())
}
