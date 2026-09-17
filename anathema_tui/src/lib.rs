mod app;
mod router;

use anathema::{
    backend::{Backend, tui::TuiBackend},
    runtime::Runtime,
    templates::Document,
};
use eyre::Result;

use crate::app::{App, AppState};

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

    builder.component(app::NAME, "templates/app.aml", App, AppState::default())?;

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
