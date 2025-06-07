use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::{
    UserId,
    commands::{Command, CommandList, CommandOutput},
    fs::FileSystem,
};

pub struct Shell {
    pub fs: FileSystem,
    pub commands: HashMap<String, CommandList>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            fs: FileSystem::default(),
            commands: CommandList::iter()
                .map(|cmd| (cmd.name().to_string(), cmd))
                .collect(),
        }
    }

    pub fn add_user(&mut self, username: &str) -> UserId {
        let group_id = self.fs.groups.add_group(username.to_string());
        let user_id = self.fs.users.add_user(username.to_string());
        let user = self.fs.users.user_mut(user_id).expect("User not found");
        user.add_group(group_id);
        user_id
    }

    pub fn new_with_user(username: &str) -> Self {
        let mut shell = Self::new();
        shell.add_user(username);
        shell
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

impl Shell {
    pub fn execute(&mut self, command: &str) -> CommandOutput {
        let fs = &mut self.fs;
        let commands = &self.commands;

        if command.is_empty() {
            panic!("Tried to execute an empty command");
        }
        let tokens = shlex::split(command).expect("Failed to split command");
        let (cmd_str, args) = tokens.split_first().expect("Failed to get command");
        let command = match commands.get(cmd_str) {
            Some(cmd) => cmd,
            None => {
                return CommandOutput(format!("Unknown command: {}", cmd_str));
            }
        };

        match command.execute(args, fs) {
            Ok(output) => output,
            Err(error) => CommandOutput(error),
        }
    }
}
