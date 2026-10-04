use actors::{
    agent::AgentHandle,
    send_to_llm::SendToLLMHandle,
    std_out_writer::StdOutWriterHandle,
    tools::{ls::LSToolHandle, read_file::ReadFileToolHandle},
};
use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use eyre::Result;
use std::{
    env::{self},
    path::PathBuf,
};

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
    let std_out_writer = StdOutWriterHandle::new();
    let read_file_tool = ReadFileToolHandle::new();
    let ls_tool = LSToolHandle::new();
    let send_to_llm = SendToLLMHandle::new(openai_client, Some(std_out_writer));
    let system_prompt = "Act as an author assistant, you have access to a semi-organize wiki containing information, chapters, and rough draftr for his multiverse. Use tools, taking your time to deep research what is needed to answer his question. Then respond appropriately.";
    let model = env::var("AI_MODEL")?;
    let writing_assistent_agent = AgentHandle::new(
        send_to_llm,
        system_prompt,
        Some(ls_tool),
        Some(read_file_tool),
        model,
    )
    .await;

    writing_assistent_agent.prompt(args.prompt).await;

    Ok(())
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short = 'p')]
    prompt: String,
    #[arg(short = 'd')]
    directory: Option<PathBuf>,
}
