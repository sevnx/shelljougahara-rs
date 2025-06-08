use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::{
    commands::{Command, CommandList, CommandOutput},
    errors::ShellError,
    fs::FileSystem,
};

pub struct Shell {
    pub fs: FileSystem,
    pub commands: HashMap<String, CommandList>,
}

impl Shell {
    pub fn new_with_user(username: &str) -> Self {
        Self {
            fs: FileSystem::new_with_user(username),
            commands: CommandList::iter()
                .map(|cmd| (cmd.name().to_string(), cmd))
                .collect(),
        }
    }
}

impl Shell {
    pub fn execute(&mut self, command: &str) -> Result<CommandOutput, ShellError> {
        let fs = &mut self.fs;
        let commands = &self.commands;

        if command.is_empty() {
            return Err(ShellError::Internal("Empty command provided".to_string()));
        }
        let tokens = shlex::split(command)
            .ok_or_else(|| ShellError::Internal("Failed to parse command".to_string()))?;
        let (cmd_str, args) = tokens
            .split_first()
            .ok_or_else(|| ShellError::Internal("Failed to get command from tokens".to_string()))?;
        let command = match commands.get(cmd_str) {
            Some(cmd) => cmd,
            None => {
                return Ok(CommandOutput(format!("Unknown command: {}", cmd_str)));
            }
        };

        match command.execute(args, fs) {
            Ok(output) => Ok(output),
            Err(error) => Err(error),
        }
    }
}
