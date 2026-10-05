use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use tokio::{
    fs::read_to_string,
    spawn,
    sync::{mpsc, oneshot},
};

struct ReadFileTool {
    receiver: mpsc::Receiver<Command>,
    name: String,
}

enum Command {
    ReadFile {
        respond_to: oneshot::Sender<Result<String, String>>,
        path: PathBuf,
    },
    GetDefinition {
        respond_to: oneshot::Sender<Value>,
    },
    GetName {
        respond_to: oneshot::Sender<String>,
    },
}

impl ReadFileTool {
    pub fn new(receiver: mpsc::Receiver<Command>) -> Self {
        let name = "read_file".to_owned();

        Self { receiver, name }
    }

    pub async fn run(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::ReadFile { respond_to, path } => {
                    respond_to
                        .send(self.handle_read_file(path).await)
                        .expect("Sending response from actor");
                }
                Command::GetDefinition { respond_to } => {
                    respond_to
                        .send(self.define_tool())
                        .expect("Sending response from actor");
                }
                Command::GetName { respond_to } => {
                    respond_to
                        .send(self.name.clone())
                        .expect("Sending response from actor");
                }
            }
        }
    }

    async fn handle_read_file(&mut self, path: PathBuf) -> Result<String, String> {
        match read_to_string(path).await {
            Ok(content) => Ok(content),
            Err(error) => {
                eprintln!("{}", format!("{error:?}").red());
                Err(format!(
                    "There was an error running the read file tool: {error}"
                ))
            }
        }
    }

    fn define_tool(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": &self.name,
                "description": "Read and return the contents of a file at the given path",
                "parameters": {
                  "type": "object",
                  "properties": {
                    "path": {
                      "type": "string",
                      "description": "The path to the file to read, this can be absolute or relative."
                    }
                  },
                  "required": ["path"]
                }
            }
        })
    }
}

#[derive(Clone)]
pub struct ReadFileToolHandle {
    sender: mpsc::Sender<Command>,
}

impl ReadFileToolHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let read_file_tool = ReadFileTool::new(receiver);

        spawn(read_file_tool.run());

        Self { sender }
    }

    pub async fn read_file(&self, args: &str) -> Result<String, String> {
        let (respond_to, recv) = oneshot::channel();
        let args: ReadFileToolArgs = match serde_json::from_str(args) {
            Ok(args) => args,
            Err(error) => {
                eprintln!("{}", format!("{error:?}").red());
                return Err(format!(
                    "There was an error parsing the tool arguments: {error:?}"
                ));
            }
        };
        println!(
            "{}",
            format!("read file tool called with args: {args:?}").green()
        );
        let command = Command::ReadFile {
            respond_to,
            path: args.path,
        };
        self.sender
            .send(command)
            .await
            .expect("Sending command to actor");

        recv.await.expect("getting response from actor")
    }

    pub async fn get_definition(&self) -> Value {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetDefinition { respond_to };
        self.sender
            .send(command)
            .await
            .expect("sending the command to the read file tool");

        recv.await
            .expect("getting responce from the read file tool")
    }

    pub async fn get_name(&self) -> String {
        let (respond_to, recv) = oneshot::channel();
        let command = Command::GetName { respond_to };
        self.sender
            .send(command)
            .await
            .expect("sending the command to the read file tool");

        recv.await
            .expect("getting responce from the read file tool")
    }
}

impl Default for ReadFileToolHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ReadFileToolArgs {
    path: PathBuf,
}
