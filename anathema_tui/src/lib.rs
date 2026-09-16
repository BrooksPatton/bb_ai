use anathema::{
    backend::{Backend, tui::TuiBackend},
    runtime::Runtime,
    templates::Document,
};
use eyre::Result;

pub fn run() -> Result<()> {
    let doc = Document::new("@app");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    builder.default::<()>("app", "templates/index.aml").unwrap();
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
