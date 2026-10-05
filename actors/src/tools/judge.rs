use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use shared_types::message::Message;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
};

struct Judge {
    receiver: mpsc::Receiver<Command>,
    name: String,
}

enum Command {
    Judge {
        respond_to: oneshot::Sender<Option<String>>,
        pass: bool,
        feedback: Option<String>,
    },
    GetDefinition {
        respond_to: oneshot::Sender<Value>,
    },
    GetName {
        respond_to: oneshot::Sender<String>,
    },
}

impl Judge {
    pub fn new(receiver: mpsc::Receiver<Command>) -> Self {
        let name = "judge".to_owned();

        Self { receiver, name }
    }

    pub async fn run(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Judge {
                    respond_to,
                    pass,
                    feedback,
                } => respond_to
                    .send(self.handle_judge(pass, feedback).await)
                    .expect("Responding to actor command"),
                Command::GetDefinition { respond_to } => respond_to
                    .send(self.define_tool())
                    .expect("Sending response to command"),
                Command::GetName { respond_to } => respond_to
                    .send(self.name.clone())
                    .expect("sending response to command"),
            }
        }
    }

    async fn handle_judge(&mut self, pass: bool, feedback: Option<String>) -> Option<String> {
        if pass { None } else { feedback }
    }

    fn define_tool(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": &self.name,
                "description": "Judge the agents work, if it is adequate, return true. Otherwise return false and give feedback for the agent to try again. You must double check the work before calling this tool.",
                "parameters": {
                  "type": "object",
                  "properties": {
                    "pass": {
                      "type": "bool",
                      "description": "did the agent do it's job properly with no need to change anything? If so this should be true."
                    },
                    "feedback": {
                        "type": "string",
                        "description": "What should the agent change to be able to pass qa check? Due to limitations of AI context history, this will be placed into the agents context history as a 'user' message. Mention who you are, and be clear about the problems and what need to be changed. Don't tell it how to do it's job, just what the problems are and why you failed the output."
                    }
                  },
                  "required": ["pass"]
                }
            }
        })
    }
}

pub struct JudgeToolHandle {
    sender: mpsc::Sender<Command>,
}

impl JudgeToolHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let judge = Judge::new(receiver);

        spawn(judge.run());

        Self { sender }
    }

    #[allow(clippy::result_large_err)]
    pub async fn judge(&self, args: &str, tool_id: String) -> Result<Option<Message>, Message> {
        let (respond_to, recv) = oneshot::channel();
        let args: JudgeArgs = match serde_json::from_str(args) {
            Ok(args) => args,
            Err(error) => {
                eprintln!("{}", format!("{error:?}").red());
                return Err(Message::new_tool(
                    format!("Error judging the agent output: {error:?}"),
                    tool_id,
                ));
            }
        };

        println!("{}", format!("Judge tool run with args: {args:?}").green());

        let command = Command::Judge {
            respond_to,
            pass: args.pass,
            feedback: args.feedback,
        };
        self.sender
            .send(command)
            .await
            .expect("Sending command to actor");

        let result = recv
            .await
            .expect("getting response from actor")
            .map(Message::new_user);

        Ok(result)
    }

    pub async fn get_definition(&self) -> Value {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetDefinition { respond_to };
        self.sender
            .send(command)
            .await
            .expect("sending the command to the ls tool");

        recv.await.expect("getting responce from the ls tool")
    }

    pub async fn get_name(&self) -> String {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetName { respond_to };
        self.sender
            .send(command)
            .await
            .expect("sending the command to the tool");

        recv.await.expect("getting responce from the tool")
    }
}

impl Default for JudgeToolHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct JudgeArgs {
    pass: bool,
    feedback: Option<String>,
}
