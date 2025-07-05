//! The touch command.

use std::path::PathBuf;

use chrono::Utc;

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags,
        args::{ArgumentKind, BasicArgument, BasicArgumentKind},
        flags::{FlagDefinition, FlagDefinitionBuilder},
    },
    errors::{FileSystemError, ShellError},
};

#[derive(Default, Clone, Copy)]
pub struct TouchCommand;

impl ExecutableCommand for TouchCommand {
    fn name(&self) -> &'static str {
        "touch"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinitionBuilder::new().into_flag_definition()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::Enumeration(BasicArgumentKind::String))
    }

    fn execute(
        &self,
        _: Flags,
        args: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let error_builder =
            |path: &str, message: &str| format!("touch: cannot touch '{path}': {message}");

        let mut current_session = shell.current_session.clone();
        let mut fs = shell.fs.clone();
        let mut error_messages = Vec::new();

        let paths = match args {
            Some(Argument::List(paths)) => {
                paths.into_iter().try_fold(Vec::new(), |mut acc, path| {
                    if let BasicArgument::String(path) = path {
                        acc.push(PathBuf::from(path));
                    } else {
                        return Err(ShellError::Internal("Invalid arguments".to_string()));
                    }
                    Ok(acc)
                })?
            }
            _ => return Err(ShellError::Internal("Invalid arguments".to_string())),
        };

        for path in paths {
            let inode = match current_session.find_inode(&fs, &path) {
                Some(inode) => inode,
                None => match current_session.create_file(&mut fs, &path) {
                    Ok(inode) => inode,
                    Err(error) => {
                        match error {
                            ShellError::FileSystem(FileSystemError::NotADirectory(_)) => {
                                error_messages.push(error_builder(
                                    &path.display().to_string(),
                                    "Not a directory",
                                ));
                            }
                            ShellError::FileSystem(FileSystemError::EntryAlreadyExists(_)) => {
                                error_messages.push(error_builder(
                                    &path.display().to_string(),
                                    "File exists",
                                ));
                            }
                            _ => {
                                return Err(error);
                            }
                        }
                        continue;
                    }
                },
            };
            let mut unlocked_inode = inode.lock().expect("Failed to lock inode");
            unlocked_inode.metadata.updated_at = Utc::now();
        }

        if error_messages.is_empty() {
            Ok(CommandOutput(None))
        } else {
            Ok(CommandOutput(Some(error_messages.join("\n"))))
        }
    }
}
