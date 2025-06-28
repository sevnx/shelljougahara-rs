//! Remove a file or directory.

use std::path::PathBuf;

use crate::{
    InodeContent, ShellError,
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags,
        args::{ArgumentKind, BasicArgument, BasicArgumentKind},
        flags::{FlagDefinition, FlagDefinitionBuilder, FlagSpecification},
    },
};

#[derive(Default, Clone, Copy)]
pub struct RemoveCommand;

impl ExecutableCommand for RemoveCommand {
    fn name(&self) -> &'static str {
        "rm"
    }

    fn flags(&self) -> FlagDefinition {
        FlagDefinitionBuilder::new()
            .with_flag(FlagSpecification::new(
                "force",
                Some('f'),
                false,
                ArgumentKind::Flag,
            ))
            .with_flag(FlagSpecification::new(
                "recursive",
                Some('r'),
                false,
                ArgumentKind::Flag,
            ))
            .into_flag_definition()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::Enumeration(BasicArgumentKind::String))
    }

    fn execute(
        &self,
        flags: Flags,
        arg: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        let mut current_session = shell.current_session.clone();
        let mut fs = shell.fs.clone();

        let force = flags.flag("force").is_some();
        let recursive = flags.flag("recursive").is_some();

        let args = match arg {
            Some(Argument::List(args)) => args,
            Some(_) => return Err(ShellError::Internal("Invalid argument".to_string())),
            None => return Err(ShellError::Internal("Missing operand".to_string())),
        };

        let mut errors = Vec::new();
        for arg in args {
            let path = match arg {
                BasicArgument::String(path) => path,
                _ => return Err(ShellError::Internal("Invalid argument".to_string())),
            };

            let path = PathBuf::from(path);
            match current_session.find_inode(&fs, &path) {
                Some(inode) => {
                    let inode = inode.lock().expect("Failed to lock inode");
                    if let InodeContent::Directory(_) = &inode.content
                        && !recursive
                        && !force
                    {
                        errors.push(format!(
                            "rm: cannot remove '{}': Is a directory",
                            path.display()
                        ));
                    } else {
                        drop(inode); // Drop the lock to avoid deadlocks
                        if let Err(error) = current_session.remove_file(&mut fs, &path) {
                            if !force {
                                errors.push(format!(
                                    "rm: cannot remove '{}': {error}",
                                    path.display()
                                ));
                            }
                        }
                    }
                }
                None if !force => {
                    errors.push(format!(
                        "rm: cannot remove '{}': No such file or directory",
                        path.display()
                    ));
                }
                None => {}
            }
        }

        Ok(CommandOutput(None))
    }
}
