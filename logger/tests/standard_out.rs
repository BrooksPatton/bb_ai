use eyre::Result;
use logger::{BBLog, BBLogDestination};

#[test]
fn should_log_to_standard_out() -> Result<()> {
    let buffer = Vec::new();
    let log_destination = BBLogDestination::StdOut(buffer);
    let mut logger = BBLog::new(log_destination);
    let message = "hello world";

    logger.log(message)?;

    let BBLogDestination::StdOut(log_buffer) = logger.buffer();
    assert_eq!(String::from_utf8(log_buffer.clone())?, message);

    Ok(())
}
