// src/main.rs
use anathema::prelude::{Backend, Document, TuiBackend};
use anathema::runtime::Runtime;
use eyre::Result;
use tui::app::{App, AppState};
use tui::components::input::{Input, InputState};
use tui::pages::home::HomePage;
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

    builder.default::<()>("connection_side", "templates/connections_side.aml")?;
    builder.default::<()>("model_modal", "templates/model_modal.aml")?;
    builder.default::<()>("router", "templates/router.aml")?;
    builder.default::<()>("top_nav", "templates/components/top_nav.aml")?;

    builder.component("app", "templates/index.aml", App, AppState::new()?)?;
    builder.component("home", "templates/pages/home.aml", HomePage, ())?;
    builder.component(
        "model_chooser",
        "templates/pages/model_chooser.aml",
        ModelChooserPage,
        ModelChooserPageState::new(),
    )?;

    builder.prototype("input", "templates/input.aml", Input::new, InputState::new)?;

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();

    Ok(())
}
