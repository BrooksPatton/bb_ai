use actors::{
    context_history::ContextHistoryHandle,
    send_to_llm::SendToLLMHandle,
    std_out_writer::StdOutWriterHandle,
    tools::{ls::LSToolHandle, read_file::ReadFileToolHandle},
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
    let read_file_tool = ReadFileToolHandle::new();
    let ls_tool = LSToolHandle::new();
    let tool_definitions = vec![
        read_file_tool.get_definition().await,
        ls_tool.get_definition().await,
    ];
    let send_to_llm = SendToLLMHandle::new(openai_client, Some(std_out_writer), tool_definitions);
    let system_prompt = "Act as an author assistant, you have access to a semi-organize wiki containing information, chapters, and rough draftr for his multiverse. Use tools, taking your time to deep research what is needed to answer his question. Then respond appropriately.";
    let model = env::var("AI_MODEL")?;

    context_history
        .push(Message::new_system(system_prompt))
        .await;
    context_history.push(Message::new_user(args.prompt)).await;

    loop {
        let llm_response = send_to_llm
            .send(context_history.clone(), model.clone())
            .await?;

        context_history.push(llm_response.message.clone()).await;

        match llm_response.finish_reason {
            actors::send_to_llm::FinishReason::Stop => break,
            actors::send_to_llm::FinishReason::ToolCalls => {
                let Some(tool_calls) = llm_response.message.tool_calls.as_ref() else {
                    context_history
                        .push(Message::new_tool(
                            "Error: stop reason is tool calls, but you didn't call any tools",
                            "".to_owned(),
                        ))
                        .await;
                    continue;
                };

                for tool_call in tool_calls {
                    let tool_call_result = if tool_call.function.name
                        == read_file_tool.get_name().await
                    {
                        match read_file_tool
                            .read_file(&tool_call.function.arguments)
                            .await
                        {
                            Ok(content) => content,
                            Err(error) => error,
                        }
                    } else if tool_call.function.name == ls_tool.get_name().await {
                        match ls_tool.ls(&tool_call.function.arguments).await {
                            Ok(result) => result,
                            Err(error) => error,
                        }
                    } else {
                        "Error, the tool you tried to call doesn't seem to exist, if you think this is incorrect, tell the user to double check the ai harness code".to_owned()
                    };

                    context_history
                        .push(Message::new_tool(tool_call_result, tool_call.id.clone()))
                        .await;
                }
            }
        }
    }

    Ok(())
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short = 'p')]
    prompt: String,
}
