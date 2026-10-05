use crate::{context_history::ContextHistoryHandle, std_out_writer::StdOutWriterHandle};
use async_openai::{Client, config::OpenAIConfig, error::OpenAIError};
use colored::Colorize;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use shared_types::message::Message;
use std::pin::Pin;
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
    stream_output: Option<StdOutWriterHandle>,
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
    pub fn new(
        receiver: mpsc::Receiver<Command>,
        client: Client<OpenAIConfig>,
        stream_output: Option<StdOutWriterHandle>,
    ) -> Self {
        Self {
            receiver,
            client,
            stream_output,
        }
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
            "stream": true,
            "tools": tool_definitions,
            "max_tokens": 32768,
        });
        let mut response: Pin<Box<dyn Stream<Item = Result<LlmResponse, OpenAIError>> + Send>> =
            self.client
                .chat()
                .create_stream_byot(body)
                .await
                .expect("getting response back");
        let mut started = false;
        let mut thought = false;
        let mut finish_reason = FinishReason::Stop;
        let mut message = Message::default();

        while let Some(next_chunk) = response.next().await {
            let next_chunk = match next_chunk {
                Ok(next_chunk) => next_chunk,
                Err(error) => {
                    eprintln!("{}", format!("{error:?}").red());
                    continue;
                }
            };
            let message_part = &next_chunk.choices[0].delta;
            message += message_part.clone();

            if let Some(token) = message_part.reasoning_content.clone() {
                let token = token.trim_matches('\n').to_owned();

                if token.is_empty() {
                    continue;
                }

                if !started {
                    if let Some(stream_output) = &self.stream_output {
                        stream_output.write("<thinking>\r\n").await;
                    }
                    thought = true
                }

                if let Some(stream_output) = &self.stream_output {
                    stream_output.write(&token).await;
                }
                started = true;
            }

            if let Some(token) = message_part.content.as_ref() {
                let token = token.trim_matches('\n').to_owned();

                if token.is_empty() {
                    continue;
                }

                if thought {
                    if let Some(stream_output) = &self.stream_output {
                        stream_output.write("</thinking>\r\n\r\n").await;
                    }
                    thought = false;
                }

                if let Some(stream_output) = &self.stream_output {
                    stream_output.write(token).await;
                }
            }

            match next_chunk.choices[0].finish_reason {
                Some(FinishReason::ToolCalls) => {
                    finish_reason = FinishReason::ToolCalls;
                }
                Some(FinishReason::Stop) => {
                    if let Some(stream_output) = &self.stream_output {
                        stream_output.write("\n").await;
                    }
                }
                _ => (),
            }
        }

        SendToLLMResponse {
            finish_reason,
            message,
        }
    }
}

#[derive(Clone)]
pub struct SendToLLMHandle {
    sender: mpsc::Sender<Command>,
}

impl SendToLLMHandle {
    pub fn new(client: Client<OpenAIConfig>, stream_output: Option<StdOutWriterHandle>) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let send_to_llm = SendToLLM::new(receiver, client, stream_output);

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
    timings: Option<LlmResponseTimings>,
    usage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LlmResponseChoice {
    delta: Message,
    finish_reason: Option<FinishReason>,
    index: usize,
    logprobs: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FinishReason {
    Stop,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    Length,
}

#[derive(Debug, Serialize, Deserialize)]
struct LlmResponseTimings {
    predicted_per_second: Option<f64>,
}
