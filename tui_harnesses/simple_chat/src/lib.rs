mod actor;
mod tui;

use crate::tui::prompt::PromptHandle;
use eyre::Result;

pub fn run() -> Result<()> {
    let prompt = PromptHandle::new();

    loop {
        let user_input = prompt.send()?;

        println!("user inputed: {user_input}");
    }
}
