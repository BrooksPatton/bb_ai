use eyre::Result;
use std::{
    fmt::Display,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub struct BBLog {
    pub std_out: Option<Box<dyn Write>>,
    pub std_err: Option<Box<dyn Write>>,
    pub file_path: Option<PathBuf>,
}

impl BBLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_std_out(mut self, writer: impl Write + 'static) -> Self {
        self.std_out = Some(Box::new(writer));
        self
    }

    pub fn with_std_err(mut self, writer: impl Write + 'static) -> Self {
        self.std_err = Some(Box::new(writer));
        self
    }

    pub fn with_file_path(mut self, path: impl AsRef<Path>) -> Self {
        self.file_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Log out to wherever we want to
    pub fn log(&mut self, message: impl Display, error: bool) -> Result<()> {
        if error {
            if let Some(writer) = &mut self.std_err {
                write!(*writer, "{message}")?;
            }
        } else {
            if let Some(writer) = &mut self.std_out {
                write!(*writer, "{message}")?;
            }
        }

        if let Some(file_path) = &self.file_path {
            let mut file = File::options()
                .write(true)
                .create(true)
                .truncate(false)
                .open(file_path)?;
            writeln!(file, "{message}")?;

            file.flush()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn canary_test() {
        assert_eq!(5, 5);
    }
}
