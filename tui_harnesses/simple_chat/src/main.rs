use eyre::Result;
use simple_chat::app;

#[tokio::main]
async fn main() -> Result<()> {
    let model = "mlx-community/Qwen3.8-27B-8bit".to_owned();
    let ai_key = "".to_owned();
    let ai_url = "http://host.docker.internal:8080".to_owned();
    let app = app::AppHandle::new(model, ai_key, ai_url);

    app.send().await?;

    Ok(())
}
