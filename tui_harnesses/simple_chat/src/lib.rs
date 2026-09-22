mod actor;
mod llm;
mod tui;

use crate::tui::prompt::PromptHandle;
use eyre::{Context, Result};
use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::{JoinHandle, spawn},
};

pub struct HarnessHandle {
    tx: Sender<HarnessCommand>,
    thread_handle: JoinHandle<()>,
}

impl HarnessHandle {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let thread_handle = Harness::spawn(rx);

        Self { tx, thread_handle }
    }

    pub fn send(&self) -> Result<()> {
        let (done_tx, done_rx) = channel();
        let command = HarnessCommand(done_tx);

        self.tx.send(command)?;

        done_rx.recv().context("sending result to caller")
    }

    pub fn is_done(&self) -> bool {
        self.thread_handle.is_finished()
    }
}

impl Default for HarnessHandle {
    fn default() -> Self {
        Self::new()
    }
}

struct Harness {}

impl Harness {
    pub fn spawn(rx: Receiver<HarnessCommand>) -> JoinHandle<()> {
        spawn(move || {
            let harness = Self {};

            while let Ok(command) = rx.recv() {
                harness
                    .handle_command(command.0)
                    .expect("handling harness command");
            }
        })
    }

    fn handle_command(&self, done: Sender<()>) -> Result<()> {
        let prompt_actor = PromptHandle::new();

        loop {
            let prompt = prompt_actor.send().expect("getting user input");

            if prompt == "/exit" {
                break;
            }
        }

        done.send(()).context("Sending response to caller")
    }
}

struct HarnessCommand(Sender<()>);
