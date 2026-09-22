use eyre::Result;
use std::{
    io::{Write, stdin, stdout},
    sync::mpsc::{Receiver, Sender, channel},
    thread::{JoinHandle, spawn},
};

pub struct PromptHandle {
    handle_tx: Sender<PromptCommand>,
}

impl PromptHandle {
    pub fn new() -> Self {
        let (handle_tx, handle_rx) = channel();
        let _handle = Prompt::spawn(handle_rx);

        Self { handle_tx }
    }

    pub fn send(&self) -> Result<String> {
        let (tx, rx) = channel();
        let command = PromptCommand::GetUserInput {
            send_response_to: tx,
        };

        self.handle_tx.send(command)?;

        let result = rx.recv()?;

        Ok(result)
    }
}

pub struct Prompt {
    handle_rx: Receiver<PromptCommand>,
    ps1: &'static str,
}

impl Prompt {
    pub fn spawn(handle_rx: Receiver<PromptCommand>) -> JoinHandle<()> {
        let ps1 = "$ ";

        spawn(move || {
            let prompt = Self { handle_rx, ps1 };

            while let Ok(command) = prompt.handle_rx.recv() {
                match command {
                    PromptCommand::GetUserInput { send_response_to } => prompt
                        .handle_get_user_input(send_response_to)
                        .expect("getting user input"),
                }
            }
        })
    }

    fn handle_get_user_input(&self, respond_to: Sender<String>) -> Result<()> {
        let mut input = String::new();

        print!("{}", self.ps1);
        stdout().flush()?;
        stdin().read_line(&mut input)?;

        respond_to.send(input.trim().to_owned())?;

        Ok(())
    }
}

pub enum PromptCommand {
    GetUserInput { send_response_to: Sender<String> },
}
