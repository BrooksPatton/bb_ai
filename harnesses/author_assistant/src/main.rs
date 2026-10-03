use actors::{
    context_history::ContextHistoryHandle, send_to_llm::SendToLLMHandle,
    std_out_writer::StdOutWriterHandle,
};
use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use eyre::Result;
use shared_types::message::Message;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::from_path(".env")?;

    let args = Args::parse();
    let api_key = env::var("OPENAI_API_KEY")?;
    let api_base = env::var("OPENAI_BASE_URL")?;
    let openai_config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(api_base);
    let openai_client = Client::with_config(openai_config);
    let context_history = ContextHistoryHandle::new();
    let std_out_writer = StdOutWriterHandle::new();
    let send_to_llm = SendToLLMHandle::new(openai_client, Some(std_out_writer));
    let system_prompt = "Act as an author assistant, you have access to a semi-organize wiki containing information, chapters, and rough draftr for his multiverse. Use tools, taking your time to deep research what is needed to answer his question. Then respond appropriately.";
    let model = env::var("AI_MODEL")?;

    context_history
        .push(Message::new_system(system_prompt))
        .await;
    context_history.push(Message::new_user(args.prompt)).await;
    let llm_response = send_to_llm.send(context_history.clone(), model).await?;

    dbg!(llm_response);

    Ok(())
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short = 'p')]
    prompt: String,
}
