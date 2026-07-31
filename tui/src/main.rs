// src/main.rs
use anathema::prelude::{Backend, Document, TuiBackend};
use anathema::runtime::Runtime;
use eyre::Result;
use tui::app::{App, AppState};
use tui::components::input::{Input, InputState};

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

    builder.component("app", "templates/index.aml", App, AppState::new()?)?;

    builder.prototype("input", "templates/input.aml", Input::new, InputState::new)?;

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();

    Ok(())
}
