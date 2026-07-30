// src/main.rs
use anathema::prelude::{Backend, Document, TuiBackend};
use anathema::runtime::Runtime;
use eyre::Result;
use tui::components::top_nav::{self, TopNav};

fn main() -> Result<()> {
    let doc = Document::new("@index");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);
    builder
        .default::<()>("index", "templates/index.aml")
        .unwrap();

    builder.template("message_history", "templates/message_history.aml")?;

    builder.component(top_nav::NAME, "templates/top_nav.aml", TopNav, ())?;

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();

    Ok(())
}
