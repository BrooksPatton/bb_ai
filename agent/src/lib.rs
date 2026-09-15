use async_openai::{Client, config::OpenAIConfig, types::responses::CreateResponseArgs};
use eyre::{Context, ContextCompat, Result};
use std::sync::mpsc::{Receiver, Sender, channel};

#[derive(Debug)]
pub struct BBAgent {
    query_rx: Receiver<BBAgentRequest>,
    query_tx: Sender<BBAgentRequest>,
    response_tx: Sender<Result<String>>,
    response_rx: Option<Receiver<Result<String>>>,
}

impl BBAgent {
    pub fn new() -> Self {
        let (query_tx, query_rx) = channel::<BBAgentRequest>();
        let (response_tx, response_rx) = channel::<Result<String>>();

        Self {
            query_rx,
            query_tx: query_tx,
            response_tx,
            response_rx: Some(response_rx),
        }
    }

    pub fn take_query_tx(&mut self) -> Sender<BBAgentRequest> {
        self.query_tx.clone()
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Ok(query) = self.query_rx.recv() {
            self.response_tx.send(self.send_request(query).await)?;
        }

        Ok(())
    }

    pub fn take_response_tx(&mut self) -> Option<Receiver<Result<String>>> {
        self.response_rx.take()
    }

    async fn send_request(&self, query: BBAgentRequest) -> Result<String> {
        let client = Client::with_config(
            OpenAIConfig::default()
                .with_api_key(query.api_key)
                .with_api_base(query.api_url),
        );
        let request = CreateResponseArgs::default()
            .model(query.model)
            .input(query.prompt)
            .build()
            .context("Building the request for ai request")?;
        let response = client.responses().create(request).await?;
        response.output_text().context("getting response text")
    }
}

pub struct BBAgentRequest {
    pub model: String,
    pub api_key: String,
    pub api_url: String,
    pub prompt: String,
}
