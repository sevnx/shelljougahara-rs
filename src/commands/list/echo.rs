//! The echo command, printing the arguments.

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags,
        args::{ArgumentKind, BasicArgument, BasicArgumentKind},
        flags::{FlagDefinition, FlagDefinitionBuilder},
    },
    errors::ShellError,
};

#[derive(Default, Clone, Copy)]
pub struct EchoCommand;

impl ExecutableCommand for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
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
        _: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError> {
        match args {
            Some(Argument::List(args)) => {
                let mut arg_strs = Vec::new();
                for arg in args {
                    match arg {
                        BasicArgument::String(arg) => arg_strs.push(arg),
                        _ => return Err(ShellError::Internal("Invalid argument".to_string())),
                    }
                }
                Ok(CommandOutput(Some(arg_strs.join(" "))))
            }
            Some(_) => Err(ShellError::Internal("Invalid argument".to_string())),
            None => Err(ShellError::Internal("Invalid argument".to_string())),
        }
    }
}
