use std::fmt::Display;

use tokio::{
    io::{AsyncWriteExt, stdout},
    sync::{mpsc, oneshot},
};

struct StdOutWriter {
    receiver: mpsc::Receiver<Command>,
}

enum Command {
    Write {
        respond_to: oneshot::Sender<()>,
        content: String,
    },
}

impl StdOutWriter {
    pub fn new(receiver: mpsc::Receiver<Command>) -> Self {
        Self { receiver }
    }

    pub async fn run(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Write {
                    respond_to,
                    content,
                } => {
                    let mut std_out = stdout();
                    std_out
                        .write_all(content.as_bytes())
                        .await
                        .expect("writing content to standard out.");
                    std_out.flush().await.expect("Flushing stdout");
                    respond_to.send(()).expect("Sending response from actor");
                }
            }
        }
    }
}

pub struct StdOutWriterHandle {
    sender: mpsc::Sender<Command>,
}

impl StdOutWriterHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let std_out_writer = StdOutWriter::new(receiver);

        tokio::spawn(std_out_writer.run());

        Self { sender }
    }

    pub async fn write(&self, content: impl Display) {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::Write {
            respond_to,
            content: content.to_string(),
        };
        self.sender
            .send(command)
            .await
            .expect("Sending command to actor");
        recv.await.expect("getting response from actor")
    }
}

impl Default for StdOutWriterHandle {
    fn default() -> Self {
        Self::new()
    }
}
