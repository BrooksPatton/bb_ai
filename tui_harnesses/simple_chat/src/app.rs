use eyre::Result;
use tokio::sync::{mpsc, oneshot};

use crate::{llm_agent::AgentActorHandle, tui::prompt::PromptHandle};

struct AppActor {
    receiver: mpsc::Receiver<AppMessage>,
    prompt_actor: PromptHandle,
    agent_actor: AgentActorHandle,
}

impl AppActor {
    fn new(
        receiver: mpsc::Receiver<AppMessage>,
        model: String,
        ai_key: String,
        ai_url: String,
    ) -> Self {
        Self {
            receiver,
            prompt_actor: PromptHandle::new(),
            agent_actor: AgentActorHandle::new(model, ai_key, ai_url),
        }
    }

    async fn handle_message(&mut self, message: AppMessage) {
        match message {
            AppMessage::Run { respond_to } => loop {
                let user_input = match self.prompt_actor.send() {
                    Ok(user_input) => user_input,
                    Err(error) => {
                        let _ = respond_to.send(Err(error));
                        return;
                    }
                };

                let agent_response = match self.agent_actor.send(user_input).await {
                    Ok(response) => response,
                    Err(error) => {
                        let _ = respond_to.send(Err(error));
                        return;
                    }
                };

                println!("{agent_response}");
            },
        }
    }

    async fn run(mut actor: AppActor) {
        while let Some(message) = actor.receiver.recv().await {
            actor.handle_message(message).await;
        }
    }
}

enum AppMessage {
    Run {
        respond_to: oneshot::Sender<Result<()>>,
    },
}

#[derive(Clone)]
pub struct AppHandle {
    sender: mpsc::Sender<AppMessage>,
}

impl AppHandle {
    pub fn new(model: String, ai_key: String, ai_url: String) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = AppActor::new(receiver, model, ai_key, ai_url);
        tokio::spawn(AppActor::run(actor));

        Self { sender }
    }

    pub async fn send(&self) -> Result<()> {
        let (done_tx, done_rx) = oneshot::channel();
        let message = AppMessage::Run {
            respond_to: done_tx,
        };
        let _ = self.sender.send(message).await;

        done_rx.await?
    }
}
