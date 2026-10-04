use shared_types::message::Message;
use std::fmt::Display;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
};

use crate::{
    context_history::ContextHistoryHandle,
    send_to_llm::SendToLLMHandle,
    tools::{ls::LSToolHandle, read_file::ReadFileToolHandle},
};

struct Agent {
    receiver: mpsc::Receiver<Command>,
    send_to_llm: SendToLLMHandle,
    context: ContextHistoryHandle,
    judge: Option<AgentHandle>,
    ls_tool: Option<LSToolHandle>,
    read_file_tool: Option<ReadFileToolHandle>,
    model: String,
}

impl Agent {
    pub fn new(
        receiver: mpsc::Receiver<Command>,
        send_to_llm: SendToLLMHandle,
        context: ContextHistoryHandle,
        judge: Option<AgentHandle>,
        ls_tool: Option<LSToolHandle>,
        read_file_tool: Option<ReadFileToolHandle>,
        model: String,
    ) -> Self {
        Self {
            receiver,
            send_to_llm,
            context,
            judge,
            ls_tool,
            read_file_tool,
            model,
        }
    }

    pub async fn run(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Prompt {
                    respond_to,
                    message,
                } => {
                    respond_to
                        .send(self.handle_prompt(message).await)
                        .expect("responding to prompt command");
                }
            }
        }
    }

    async fn handle_prompt(&mut self, message: Message) -> AgentResponse {
        let mut tool_definitions = Vec::new();

        self.context.push(message).await;
        if let Some(read_file) = self.read_file_tool.as_ref() {
            tool_definitions.push(read_file.get_definition().await);
        }
        if let Some(ls) = self.ls_tool.as_ref() {
            tool_definitions.push(ls.get_definition().await);
        }

        self.send_to_llm
            .send(self.context.clone(), self.model.clone(), tool_definitions)
            .await
            .expect("Sending message to llm");

        AgentResponse {}
    }
}

enum Command {
    Prompt {
        respond_to: oneshot::Sender<AgentResponse>,
        message: Message,
    },
}

#[derive(Debug)]
pub struct AgentResponse {}

pub struct AgentHandle {
    sender: mpsc::Sender<Command>,
}

impl AgentHandle {
    pub async fn new(
        send_to_llm: SendToLLMHandle,
        system_prompt: impl Display,
        ls_tool: Option<LSToolHandle>,
        read_file_tool: Option<ReadFileToolHandle>,
        model: String,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let system_message = Message::new_system(system_prompt);
        let context = ContextHistoryHandle::new();
        let judge = None;
        let agent = Agent::new(
            receiver,
            send_to_llm,
            context.clone(),
            judge,
            ls_tool,
            read_file_tool,
            model,
        );

        context.push(system_message).await;
        spawn(agent.run());

        Self { sender }
    }

    pub async fn prompt(&self, prompt: impl Display) -> AgentResponse {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::Prompt {
            respond_to,
            message: Message::new_user(prompt),
        };

        self.sender
            .send(command)
            .await
            .expect("sending command to agent");

        recv.await.expect("getting response from agent actor")
    }
}
