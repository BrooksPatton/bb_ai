use shared_types::message::Message;
use tokio::{
    spawn,
    sync::{
        mpsc,
        oneshot::{self},
    },
};

struct ContextHistory {
    receiver: mpsc::Receiver<Command>,
    history: Vec<Message>,
}

enum Command {
    Push {
        respond_to: oneshot::Sender<()>,
        message: Message,
    },
    GetAll {
        respond_to: oneshot::Sender<Vec<Message>>,
    },
}

impl ContextHistory {
    pub fn new(receiver: mpsc::Receiver<Command>) -> Self {
        let history = Vec::new();

        Self { receiver, history }
    }

    pub async fn run(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Push {
                    respond_to,
                    message,
                } => {
                    self.history.push(message);
                    if let Err(error) = respond_to.send(()) {
                        eprintln!("{error:?}");
                    }
                }
                Command::GetAll { respond_to } => {
                    if let Err(error) = respond_to.send(self.history.clone()) {
                        eprintln!("{error:?}");
                    }
                }
            }
        }

        println!("Context History actor closed");
    }
}

#[derive(Clone)]
pub struct ContextHistoryHandle {
    sender: mpsc::Sender<Command>,
}

impl ContextHistoryHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let context_history = ContextHistory::new(receiver);

        spawn(context_history.run());

        Self { sender }
    }

    pub async fn push(&self, message: Message) {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::Push {
            respond_to,
            message,
        };
        if let Err(error) = self.sender.send(command).await {
            eprintln!("{error:?}");
        }
        if let Err(error) = recv.await {
            eprintln!("{error:?}");
        }
    }

    pub async fn get_all(&self) -> Vec<Message> {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetAll { respond_to };
        if let Err(error) = self.sender.send(command).await {
            eprintln!("{error:?}");
        }

        recv.await.expect("getting all messages from history")
    }
}

impl Default for ContextHistoryHandle {
    fn default() -> Self {
        Self::new()
    }
}
