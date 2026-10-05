use crate::context_history::ContextHistoryHandle;
use async_openai::{Client, config::OpenAIConfig};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use shared_types::message::Message;
use tokio::{
    spawn,
    sync::{
        mpsc,
        oneshot::{self, error::RecvError},
    },
};

struct SendToLLM {
    receiver: mpsc::Receiver<Command>,
    client: Client<OpenAIConfig>,
}

enum Command {
    Send {
        respond_to: oneshot::Sender<SendToLLMResponse>,
        context_history: ContextHistoryHandle,
        model: String,
        tool_definitions: Vec<Value>,
    },
}

#[derive(Debug)]
pub struct SendToLLMResponse {
    pub finish_reason: FinishReason,
    pub message: Message,
}

impl SendToLLM {
    pub fn new(receiver: mpsc::Receiver<Command>, client: Client<OpenAIConfig>) -> Self {
        Self { receiver, client }
    }

    pub async fn run(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Send {
                    respond_to,
                    context_history,
                    model,
                    tool_definitions,
                } => {
                    let response = self
                        .handle_send(context_history, model, tool_definitions)
                        .await;
                    respond_to.send(response).expect("Responding from actor");
                }
            }
        }
    }

    async fn handle_send(
        &mut self,
        context_history: ContextHistoryHandle,
        model: String,
        tool_definitions: Vec<Value>,
    ) -> SendToLLMResponse {
        let messages = context_history.get_all().await;
        let body = json! ({
            "messages": messages,
            "model": model,
            "stream": false,
            "tools": tool_definitions,
            "max_tokens": 32768,
        });
        let response: LlmResponse = self
            .client
            .chat()
            .create_byot(body)
            .await
            .expect("getting response back");
        let LlmResponseChoice {
            message,
            finish_reason,
            ..
        } = &response.choices[0];

        println!("{message}");

        SendToLLMResponse {
            finish_reason: *finish_reason,
            message: message.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SendToLLMHandle {
    sender: mpsc::Sender<Command>,
}

impl SendToLLMHandle {
    pub fn new(client: Client<OpenAIConfig>) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let send_to_llm = SendToLLM::new(receiver, client);

        spawn(send_to_llm.run());

        Self { sender }
    }

    pub async fn send(
        &self,
        context_history: ContextHistoryHandle,
        model: String,
        tool_definitions: Vec<Value>,
    ) -> Result<SendToLLMResponse, RecvError> {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::Send {
            respond_to,
            context_history,
            model,
            tool_definitions,
        };
        self.sender
            .send(command)
            .await
            .expect("Sending command to actor");

        recv.await
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LlmResponse {
    choices: Vec<LlmResponseChoice>,
    created: usize,
    id: String,
    model: String,
    object: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LlmResponseChoice {
    message: Message,
    finish_reason: FinishReason,
    index: usize,
    logprobs: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum FinishReason {
    Stop,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    Length,
}
