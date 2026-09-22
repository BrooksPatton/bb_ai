use eyre::Result;
use simple_chat::HarnessHandle;

fn main() -> Result<()> {
    let harness = HarnessHandle::new();
    harness.send()?;

    Ok(())
}
