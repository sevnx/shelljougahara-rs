//! The shell, the main entry point for the shell.

use crate::{
    commands::{Command, CommandOutput, get_commands},
    errors::ShellError,
    fs::FileSystem,
};

pub struct Shell {
    pub fs: FileSystem,
    pub executed_commands: Vec<String>,
    pub active: bool,
}

impl Shell {
    pub fn new_with_user(username: &str) -> Self {
        Self {
            fs: FileSystem::new_with_user(username),
            executed_commands: Vec::new(),
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
            .ok_or_else(|| ShellError::Internal(format!("Unknown command: {}", cmd_str)))?;
        self.executed_commands.push(command.name().to_string());

        match command.execute(args, self) {
            Ok(output) => Ok(output),
            Err(error) => Err(error),
        }
    }
}
