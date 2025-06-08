//! The print working directory command.

use crate::{
    FileSystem,
    commands::{Command, CommandOutput, flags::Flags},
    errors::CommandError,
};

#[derive(Default)]
pub struct PwdCommand;

impl Command for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn flags(&self) -> Flags {
        // TODO: Maybe add `-L` and `-P` flags.
        Flags::new()
    }

    fn execute(&self, _: &[String], fs: &mut FileSystem) -> Result<CommandOutput, CommandError> {
        let path = fs.current_dir.upgrade().ok_or_else(|| {
            error!("Current directory does not exist");
            "Internal error: current directory does not exist"
        })?;

        let path = path.borrow().path()?;

        Ok(CommandOutput(path.display().to_string()))
    }
}
