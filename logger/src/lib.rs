use eyre::Result;
use std::{fmt::Display, io::Write};

pub struct BBLog<W: Write + Clone> {
    destination: BBLogDestination<W>,
}

impl<W: Write + Clone> BBLog<W> {
    pub fn new(destination: BBLogDestination<W>) -> Self {
        Self { destination }
    }
    pub fn log(&mut self, message: impl Display) -> Result<()> {
        match &mut self.destination {
            BBLogDestination::StdOut(buffer) => write!(buffer, "{message}")?,
        }
        Ok(())
    }

    pub fn buffer(&self) -> &BBLogDestination<W> {
        &self.destination
    }
}

#[derive(Clone)]
pub enum BBLogDestination<W: Write + Clone> {
    StdOut(W),
}

#[cfg(test)]
mod tests {
    #[test]
    fn canary_test() {
        assert_eq!(5, 5);
    }
}
