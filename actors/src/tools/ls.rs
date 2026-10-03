use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use tokio::{
    fs::read_to_string,
    spawn,
    sync::{mpsc, oneshot},
};

struct LSTool {
    receiver: mpsc::Receiver<Command>,
    name: String,
}

enum Command {
    LS {
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

impl LSTool {
    pub fn new(receiver: mpsc::Receiver<Command>) -> Self {
        let name = "ls".to_owned();

        Self { receiver, name }
    }

    pub async fn run(mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::LS { respond_to, path } => {
                    if let Err(error) = respond_to.send(self.handle_ls(path).await) {
                        eprintln!("{error:?}");
                    }
                }
                Command::GetDefinition { respond_to } => {
                    if let Err(error) = respond_to.send(self.define_tool()) {
                        eprintln!("{error:?}");
                    }
                }
                Command::GetName { respond_to } => {
                    if let Err(error) = respond_to.send(self.name.clone()) {
                        eprintln!("{error:?}");
                    }
                }
            }
        }
    }

    async fn handle_ls(&mut self, path: PathBuf) -> Result<String, String> {
        match tokio::fs::read_dir(path).await {
            Ok(mut read_dir) => {
                let mut entries = vec![];

                while let Ok(Some(dir_entry)) = read_dir.next_entry().await {
                    let dir_entry_path = dir_entry.path();
                    let dir_entry_name = dir_entry.file_name();
                    let name = match dir_entry_name.into_string() {
                        Ok(name) => name,
                        Err(error) => {
                            eprintln!("{error:?}");
                            continue;
                        }
                    };
                    let metadata = match dir_entry.metadata().await {
                        Ok(metadata) => metadata,
                        Err(error) => {
                            eprintln!("{error:?}");
                            continue;
                        }
                    };
                    let is_dir = metadata.is_dir();
                    let response = json!({
                        "name": name,
                        "path": dir_entry_path,
                        "is_dir": &is_dir,
                    })
                    .to_string();

                    entries.push(response);
                }

                Ok(format!("{entries:?}"))
            }
            Err(error) => {
                eprintln!("{error:?}");
                Err(format!("There was an error running the ls tool: {error:?}"))
            }
        }
    }

    fn define_tool(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": &self.name,
                "description": "Run ls (list file and directories) at the given path. You will get a list of file/directory names, paths, and if they are a directory or not.",
                "parameters": {
                  "type": "object",
                  "properties": {
                    "path": {
                      "type": "string",
                      "description": "The path to run ls at, this can be absolute or relative."
                    }
                  },
                  "required": ["path"]
                }
            }
        })
    }
}

pub struct LSToolHandle {
    sender: mpsc::Sender<Command>,
}

impl LSToolHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let ls_tool = LSTool::new(receiver);

        spawn(ls_tool.run());

        Self { sender }
    }

    pub async fn ls(&self, args: &str) -> Result<String, String> {
        let (respond_to, recv) = oneshot::channel();
        let args: LSToolArgs = match serde_json::from_str(args) {
            Ok(args) => args,
            Err(error) => {
                eprintln!("{error:?}");
                return Err(format!(
                    "There was an error parsing the tool arguments: {error:?}"
                ));
            }
        };
        let command = Command::LS {
            respond_to,
            path: args.path,
        };
        if let Err(error) = self.sender.send(command).await {
            eprintln!("{error:?}");
            return Err("There was an undefined error sending a message to the function running the tool. The ls tool seems to be broken unfortunately".to_owned());
        }

        match recv.await {
            Ok(file_content) => file_content,
            Err(error) => {
                eprintln!("{error:?}");
                Err("There was a problem getting the response back from the function running the ls tool.".to_owned())
            }
        }
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

impl Default for LSToolHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct LSToolArgs {
    path: PathBuf,
}
