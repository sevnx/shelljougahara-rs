//! The shell structure, the main unit of the shell environment.

use std::path::PathBuf;

use crate::{
    commands::{Command, CommandOutput, get_commands},
    errors::ShellError,
    fs::FileSystem,
    sessions::Session,
};

#[derive(Debug, Clone)]
pub struct Shell {
    pub fs: FileSystem,
    pub current_session: Session,
    pub active: bool,
}

impl Shell {
    #[must_use] pub fn new_with_user(username: &str) -> Self {
        let mut fs = FileSystem::new();
        let user_id = fs.add_user(username).expect("Failed to add user");
        let current_session = Session::new(PathBuf::from(format!("/home/{username}")), user_id);

        Self {
            fs,
            current_session,
            active: true,
        }
    }
}

impl Shell {
    pub fn execute(&mut self, command: &str) -> Result<CommandOutput, ShellError> {
        if !self.active {
            return Err(ShellError::ShellNotActive);
        }
        if command.is_empty() {
            return Err(ShellError::Internal("Empty command provided".to_string()));
        }
        let tokens = shlex::split(command)
            .ok_or_else(|| ShellError::Internal("Failed to parse command".to_string()))?;
        let (cmd_str, args) = tokens
            .split_first()
            .ok_or_else(|| ShellError::Internal("Failed to get command from tokens".to_string()))?;

        let command = get_commands()
            .get(cmd_str.as_str())
            .ok_or_else(|| ShellError::Internal(format!("Unknown command: {cmd_str}")))?;

        self.current_session.add_to_history(command.name());

        match command.execute(args, self) {
            Ok(output) => Ok(output),
            Err(error) => Err(error),
        }
    }
}
