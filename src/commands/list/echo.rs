//! The echo command, printing the arguments.

use crate::{
    commands::{
        Argument, CommandOutput, ExecutableCommand, Flags, args::ArgumentKind,
        flags::FlagDefinition,
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
        FlagDefinition::new()
    }

    fn args(&self) -> Option<ArgumentKind> {
        Some(ArgumentKind::Enumeration(Box::new(ArgumentKind::String)))
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
                    let arg_str = match arg {
                        Argument::String(arg) => arg,
                        _ => return Err(ShellError::Internal("Invalid argument".to_string())),
                    };
                    arg_strs.push(arg_str);
                }
                Ok(CommandOutput(Some(arg_strs.join(" "))))
            }
            Some(_) => Err(ShellError::Internal("Invalid argument".to_string())),
            None => Err(ShellError::Internal("Invalid argument".to_string())),
        }
    }
}
