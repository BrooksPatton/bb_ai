use std::io::{Write, stdin, stdout};

use eyre::Result;
use logger::BBLog;

pub struct SimpleChatHarness {
    logger: BBLog,
}

impl SimpleChatHarness {
    pub fn new(logger: BBLog) -> Self {
        Self { logger }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.print_prompt()?;
            let user_input = self.get_user_input()?;
            self.logger.log(format!("USER: {user_input}"), false)?;
        }
    }

    fn print_prompt(&self) -> Result<()> {
        print!("$ ");
        stdout().flush()?;

        Ok(())
    }

    fn get_user_input(&self) -> Result<String> {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input)?;

        Ok(user_input.trim().to_owned())
    }
}
