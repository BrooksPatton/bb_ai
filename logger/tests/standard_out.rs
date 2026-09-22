use eyre::Result;
use logger::BBLog;
use std::{cell::RefCell, fs::read_to_string, io::Write, ops::Deref, rc::Rc};

#[derive(Clone, Default)]
struct MockBuffer(Rc<RefCell<Vec<u8>>>);

impl Write for MockBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.borrow_mut().flush()
    }
}

#[test]
fn should_log_to_standard_out() -> Result<()> {
    let buffer = MockBuffer::default();
    let mut logger = BBLog::new().with_std_out(buffer.clone());
    let message = "hello world";

    logger.log(message, false)?;

    assert_eq!(str::from_utf8(buffer.0.borrow().deref())?, message);

    Ok(())
}
#[test]
fn should_log_to_standard_error() -> Result<()> {
    let buffer = MockBuffer::default();
    let mut logger = BBLog::new().with_std_err(buffer.clone());
    let message = "hello world";

    logger.log(message, true)?;

    assert_eq!(str::from_utf8(buffer.0.borrow().deref())?, message);

    Ok(())
}

#[test]
fn should_log_to_file() -> Result<()> {
    let file_path = "test_log_file.log";
    let mut logger = BBLog::new().with_file(file_path, true)?;
    let message = "Hello world";

    logger.log(message, false)?;

    let log_file_contents = read_to_string(file_path)?;

    assert_eq!(log_file_contents.trim(), message);

    Ok(())
}
