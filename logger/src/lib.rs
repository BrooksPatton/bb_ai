use eyre::Result;
use std::{fmt::Display, fs::File, io::Write, path::Path};

#[derive(Default)]
pub struct BBLog {
    pub std_out: Option<Box<dyn Write>>,
    pub std_err: Option<Box<dyn Write>>,
    pub file_handle: Option<File>,
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

    pub fn with_file(mut self, path: impl AsRef<Path>, truncate: bool) -> Result<Self> {
        let file = if truncate {
            File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?
        } else {
            File::options().create(true).append(true).open(path)?
        };

        self.file_handle = Some(file);

        Ok(self)
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

        if let Some(file) = &mut self.file_handle {
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
