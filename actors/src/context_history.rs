use colored::Colorize;
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
    GetLast {
        respond_to: oneshot::Sender<Option<Message>>,
    },
    GetAllWithContent {
        respond_to: oneshot::Sender<Vec<Message>>,
    },
    GetLastWithContent {
        respond_to: oneshot::Sender<Option<Message>>,
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
                    respond_to.send(()).expect("sending response from actor");
                }
                Command::GetAll { respond_to } => {
                    respond_to
                        .send(self.history.clone())
                        .expect("sending response from actor");
                }
                Command::GetLast { respond_to } => {
                    respond_to
                        .send(self.history.last().cloned())
                        .expect("sending response from ");
                }
                Command::GetAllWithContent { respond_to } => {
                    let messages_with_content = self
                        .history
                        .iter()
                        .filter(|message| {
                            message
                                .content
                                .as_ref()
                                .is_some_and(|content| !content.is_empty())
                        })
                        .cloned()
                        .collect();
                    respond_to
                        .send(messages_with_content)
                        .expect("sending response from actor");
                }
                Command::GetLastWithContent { respond_to } => respond_to
                    .send(self.handle_get_last_with_context().await)
                    .expect("Sending resonse to get last with content command"),
            }
        }

        println!("{}", "Context History actor closed".blue());
    }

    async fn handle_get_last_with_context(&self) -> Option<Message> {
        self.history
            .iter()
            .rfind(|message| {
                message
                    .content
                    .as_ref()
                    .is_some_and(|content| !content.is_empty())
                    && message.role == shared_types::role::Role::Assistant
            })
            .cloned()
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
        self.sender
            .send(command)
            .await
            .expect("sending command to actor");
        recv.await.expect("Getting response from actor")
    }

    pub async fn get_all(&self) -> Vec<Message> {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetAll { respond_to };
        self.sender
            .send(command)
            .await
            .expect("Sending command to actor");

        recv.await.expect("getting all messages from history")
    }

    pub async fn get_last(&self) -> Option<Message> {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetLast { respond_to };
        self.sender
            .send(command)
            .await
            .expect("Sending command to actor");

        recv.await.expect("getting response from actor")
    }

    pub async fn get_messages_with_content(&self) -> Vec<Message> {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetAllWithContent { respond_to };
        self.sender
            .send(command)
            .await
            .expect("Sending command to actor");

        recv.await.expect("getting response from actor")
    }

    pub async fn get_last_with_content(&self) -> Option<Message> {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetLastWithContent { respond_to };

        self.sender
            .send(command)
            .await
            .expect("sending get last with content command to actor");

        recv.await
            .expect("getting last message with content from context history actor")
    }
}

impl Default for ContextHistoryHandle {
    fn default() -> Self {
        Self::new()
    }
}
