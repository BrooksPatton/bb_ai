use crate::{
    context_history::ContextHistoryHandle,
    send_to_llm::SendToLLMHandle,
    tools::{judge::JudgeToolHandle, ls::LSToolHandle, read_file::ReadFileToolHandle},
};
use colored::Colorize;
use shared_types::message::Message;
use std::fmt::Display;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
};

struct Agent {
    receiver: mpsc::Receiver<Command>,
    send_to_llm: SendToLLMHandle,
    context: ContextHistoryHandle,
    judge: Option<AgentHandle>,
    ls_tool: Option<LSToolHandle>,
    read_file_tool: Option<ReadFileToolHandle>,
    model: String,
    name: String,
    judge_tool: Option<JudgeToolHandle>,
}

#[allow(clippy::too_many_arguments)]
impl Agent {
    pub fn new(
        receiver: mpsc::Receiver<Command>,
        send_to_llm: SendToLLMHandle,
        context: ContextHistoryHandle,
        judge: Option<AgentHandle>,
        ls_tool: Option<LSToolHandle>,
        read_file_tool: Option<ReadFileToolHandle>,
        model: String,
        name: String,
        judge_tool: Option<JudgeToolHandle>,
    ) -> Self {
        Self {
            receiver,
            send_to_llm,
            context,
            judge,
            ls_tool,
            read_file_tool,
            model,
            name,
            judge_tool,
        }
    }

    pub async fn run(mut self) {
        println!("{}", format!("{} running", self.name).blue());

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
                Command::AddMessageToContext {
                    respond_to,
                    message,
                } => {
                    self.handle_add_message_to_context(message).await;
                    respond_to.send(()).expect("Responding to command");
                }
            }
        }
    }

    async fn handle_prompt(&mut self, prompt: Message) -> AgentResponse {
        self.context.push(prompt.clone()).await;

        let mut tool_definitions = Vec::new();
        if let Some(read_file) = self.read_file_tool.as_ref() {
            tool_definitions.push(read_file.get_definition().await);
        }
        if let Some(ls) = self.ls_tool.as_ref() {
            tool_definitions.push(ls.get_definition().await);
        }
        if let Some(judge_tool) = self.judge_tool.as_ref() {
            tool_definitions.push(judge_tool.get_definition().await);
        }

        loop {
            println!("{}", format!("Agent {} looping", self.name).blue());
            let result = self
                .send_to_llm
                .send(
                    self.context.clone(),
                    self.model.clone(),
                    tool_definitions.clone(),
                )
                .await
                .expect("Sending message to llm");

            self.context.push(result.message.clone()).await;

            match result.finish_reason {
                crate::send_to_llm::FinishReason::Stop => {
                    let deliverable = result.message.clone();

                    if let Some(judge) = &self.judge {
                        println!("{}", format!("{deliverable}").blue());

                        let judge_result = judge
                            .prompt(format!(
                                "full message history sent to agent: ```{:?}```\n\ndeliverable: ```{deliverable}```",

                                self.context.get_all().await
                            ))
                            .await;

                        if judge_result.finished {
                            println!("{}", "Judge determined we're done".blue());

                            return AgentResponse {
                                finished: true,
                                message: deliverable,
                            };
                        } else {
                            println!(
                                "{} {}",
                                "Judge determined we need to try again: ".blue(),
                                judge_result.message
                            );

                            self.context.push(judge_result.message).await;
                            continue;
                        }
                    }

                    return AgentResponse {
                        finished: true,
                        message: deliverable,
                    };
                }
                crate::send_to_llm::FinishReason::ToolCalls => {
                    let Some(tool_calls) = result.message.tool_calls.as_ref() else {
                        self.context.push(Message::new_tool("Stop reason was tool calls, but there weren't any tools called. Please try again.", "".to_owned())).await;
                        continue;
                    };

                    for tool_call in tool_calls {
                        let tool_call_name = &tool_call.function.name;
                        if let Some(read_file) = self.read_file_tool.as_ref()
                            && read_file.get_name().await == *tool_call_name
                        {
                            let read_file_result =
                                match read_file.read_file(&tool_call.function.arguments).await {
                                    Ok(message) => message,
                                    Err(message) => message,
                                };
                            self.context
                                .push(Message::new_tool(read_file_result, tool_call.id.clone()))
                                .await;
                        } else if let Some(ls) = self.ls_tool.as_ref()
                            && ls.get_name().await == *tool_call_name
                        {
                            let ls_result = match ls.ls(&tool_call.function.arguments).await {
                                Ok(message) => message,
                                Err(message) => message,
                            };
                            self.context
                                .push(Message::new_tool(ls_result, tool_call.id.clone()))
                                .await;
                        } else if let Some(judge_tool) = self.judge_tool.as_ref()
                            && judge_tool.get_name().await == *tool_call_name
                        {
                            let judge_result = match judge_tool
                                .judge(&tool_call.function.arguments, tool_call.id.clone())
                                .await
                            {
                                Ok(Some(message)) => message,
                                Err(message) => message,
                                Ok(None) => {
                                    let agent_response = AgentResponse {
                                        finished: true,
                                        message: self
                                            .context
                                            .get_last_with_content()
                                            .await
                                            .unwrap_or_else(|| Message::new_assistant("")),
                                    };

                                    return agent_response;
                                }
                            };

                            self.context.push(judge_result).await;
                        }
                    }
                }
                crate::send_to_llm::FinishReason::Length => {
                    self.context.push(result.message).await;
                    continue;
                }
            }
        }
    }

    async fn handle_add_message_to_context(&self, message: Message) {
        self.context.push(message).await
    }
}

enum Command {
    Prompt {
        respond_to: oneshot::Sender<AgentResponse>,
        message: Message,
    },
    AddMessageToContext {
        respond_to: oneshot::Sender<()>,
        message: Message,
    },
}

#[derive(Debug)]
pub struct AgentResponse {
    finished: bool,
    message: Message,
}

pub struct AgentHandle {
    sender: mpsc::Sender<Command>,
}

#[allow(clippy::too_many_arguments)]
impl AgentHandle {
    pub async fn new(
        send_to_llm: SendToLLMHandle,
        system_prompt: impl Display,
        ls_tool: Option<LSToolHandle>,
        read_file_tool: Option<ReadFileToolHandle>,
        model: String,
        judge: Option<AgentHandle>,
        name: impl Display,
        judge_tool: Option<JudgeToolHandle>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let system_message = Message::new_system(system_prompt);
        let context = ContextHistoryHandle::new();
        let agent = Agent::new(
            receiver,
            send_to_llm,
            context.clone(),
            judge,
            ls_tool,
            read_file_tool,
            model,
            name.to_string(),
            judge_tool,
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

    pub async fn add_message_to_context(&self, message: Message) {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::AddMessageToContext {
            respond_to,
            message,
        };

        self.sender
            .send(command)
            .await
            .expect("sending command to agent");

        recv.await.expect("getting response from agent actor")
    }
}
