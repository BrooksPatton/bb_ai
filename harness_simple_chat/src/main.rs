use eyre::Result;
use harness_simple_chat::SimpleChatHarness;
use logger::BBLog;

fn main() -> Result<()> {
    let logger = BBLog::new().with_file("./harness_simple_chat.log", false)?;
    let mut harness = SimpleChatHarness::new(logger);
    harness.run()
}
