use std::fmt::Display;

use async_openai::config::OpenAIConfig;
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

struct AgentActor {
    receiver: mpsc::Receiver<AgentActorMessage>,
    model: String,
    client: async_openai::Client<OpenAIConfig>,
    messages: Vec<AIMessage>,
}

enum AgentActorMessage {
    GenerateResponse {
        respond_to: oneshot::Sender<AgentActorResponse>,
        user_input: String,
    },
}

impl AgentActor {
    pub fn new(
        receiver: mpsc::Receiver<AgentActorMessage>,
        model: String,
        ai_key: String,
        ai_url: String,
    ) -> Self {
        let config = async_openai::config::OpenAIConfig::default()
            .with_api_key(ai_key)
            .with_api_base(ai_url);
        let client = async_openai::Client::with_config(config);
        let messages = Vec::new();

        Self {
            receiver,
            model,
            client,
            messages,
        }
    }

    async fn handle_message(&mut self, message: AgentActorMessage) {
        match message {
            AgentActorMessage::GenerateResponse {
                respond_to,
                user_input,
            } => {
                self.messages.push(AIMessage::new_user(user_input));

                let raw_response: Value = self
                    .client
                    .chat()
                    .create_byot(serde_json::json!({
                        "messages": &self.messages,
                        "model": &self.model,
                        "stream": false,
                        "enable_thinking": true
                    }))
                    .await
                    .expect("sending request to ai api");

                dbg!(&raw_response);

                let ai_response: AIResponse = serde_json::from_value(raw_response)
                    .expect("converting raw response from ai to typed value");
                let ai_message = &ai_response.choices[0].message;

                self.messages.push(ai_message.clone());

                let _ = respond_to.send(AgentActorResponse::Assistant {
                    message: ai_message.content.clone(),
                    role: ai_message.role,
                });
            }
        }
    }

    pub async fn run(mut actor: Self) {
        while let Some(message) = actor.receiver.recv().await {
            actor.handle_message(message).await;
        }
    }
}

#[derive(Clone)]
pub struct AgentActorHandle {
    sender: mpsc::Sender<AgentActorMessage>,
}

impl AgentActorHandle {
    pub fn new(model: String, ai_key: String, ai_url: String) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let agent_actor = AgentActor::new(receiver, model, ai_key, ai_url);

        tokio::spawn(AgentActor::run(agent_actor));

        Self { sender }
    }

    pub async fn send(&self, user_input: String) -> Result<AgentActorResponse> {
        let (respond_to_tx, respond_to_rx) = oneshot::channel();
        let message = AgentActorMessage::GenerateResponse {
            respond_to: respond_to_tx,
            user_input,
        };

        let _ = self.sender.send(message).await;

        respond_to_rx
            .await
            .context("getting response from agent actor")
    }
}

#[derive(Debug, Deserialize)]
struct AIResponse {
    choices: Vec<AIResponseChoice>,
}

#[derive(Debug, Deserialize)]
struct AIResponseChoice {
    message: AIMessage,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct AIMessage {
    content: String,
    role: AIRole,
}

impl AIMessage {
    pub fn new_user(content: String) -> Self {
        Self {
            content,
            role: AIRole::User,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AIRole {
    Assistant,
    User,
}

pub enum AgentActorResponse {
    Assistant { message: String, role: AIRole },
}

impl Display for AgentActorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assistant { message, role } => write!(f, "{role:?}::{message}"),
        }
    }
}
