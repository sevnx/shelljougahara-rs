//! Representations of commands, elements that can be executed in the shell context.

use std::{collections::HashMap, sync::OnceLock};

use crate::{
    commands::args::{ArgumentKind, BasicArgument, parse_string_argument},
    errors::ShellError,
};
use flags::FlagDefinition;
use strum::IntoEnumIterator;

pub use args::Argument;
pub use flags::Flags;
pub use list::Commands;

mod args;
mod flags;
mod list;

static COMMANDS: OnceLock<HashMap<&str, Commands>> = OnceLock::new();

pub fn get_commands() -> &'static HashMap<&'static str, Commands> {
    COMMANDS.get_or_init(|| Commands::iter().map(|cmd| (cmd.name(), cmd)).collect())
}

#[enum_dispatch::enum_dispatch(Commands)]
pub trait ExecutableCommand {
    /// The name of the command
    fn name(&self) -> &'static str;

    /// The flags of the command
    fn flags(&self) -> FlagDefinition;

    /// The arguments of the command
    fn args(&self) -> Option<ArgumentKind>;

    /// Executes a command
    ///
    /// # Returns
    ///
    /// - `Ok(CommandOutput)` on successful execution, where the output contains
    ///   the command's result or any user-facing error messages.
    /// - `Err` when an unexpected internal error occurs that prevents command execution.
    fn execute(
        &self,
        flags: Flags,
        args: Option<Argument>,
        shell: &mut crate::shell::Shell,
    ) -> Result<CommandOutput, ShellError>;
}

#[derive(Debug)]
pub struct CommandOutput(pub Option<String>);

pub struct Command {
    pub command: Commands,
    pub flags: Flags,
    pub args: Option<Argument>,
}

impl Command {
    pub fn from_tokens(tokens: Vec<String>) -> Result<Self, Error> {
        let (cmd_str, args) = tokens
            .split_first()
            .ok_or_else(|| Error::Internal("Failed to get command from tokens".to_string()))?;

        let command = get_commands()
            .get(cmd_str.as_str())
            .ok_or_else(|| Error::UnknownCommand(cmd_str.to_string()))?;

        let command_parser =
            CommandParser::new(command, command.flags(), command.args(), args.to_vec());
        command_parser.parse()
    }
}

pub struct CommandParser {
    command: &'static Commands,
    flag_defs: FlagDefinition,
    arg_defs: Option<ArgumentKind>,
    args: Vec<String>,
    parsed_flags: Flags,
    parsed_args: Option<Argument>,
}

impl CommandParser {
    pub fn new(
        command: &'static Commands,
        flag_defs: FlagDefinition,
        flag_args: Option<ArgumentKind>,
        args: Vec<String>,
    ) -> Self {
        Self {
            command,
            flag_defs,
            arg_defs: flag_args,
            args,
            parsed_flags: Flags::new(),
            parsed_args: None,
        }
    }

    pub fn parse(mut self) -> Result<Command, Error> {
        let mut args_iter = self.args.iter().peekable();
        while let Some(arg) = args_iter.next() {
            match arg_kind(arg) {
                ArgKind::ShorthandFlag => {
                    let mut flag_iter = arg.chars().skip(1).peekable();
                    while let Some(flag) = flag_iter.next() {
                        let has_next_flag = flag_iter.peek().is_some();
                        let has_next_arg = args_iter.peek().is_some();
                        match self.flag_defs.get_flag_shorthand(flag) {
                            Some(flag_spec) => match &flag_spec.arg_type {
                                ArgumentKind::Basic(_) => {
                                    if has_next_flag {
                                        // Technically this means that we combined flags but this
                                        // one expected an argument.
                                        // TODO: Search up what is the proper sanction for this.
                                        return Err(Error::InvalidFlagArgument(
                                            flag.to_string(),
                                            "No argument provided".to_string(),
                                        ));
                                    }
                                    if !has_next_arg {
                                        return Err(Error::InvalidFlagArgument(
                                            flag.to_string(),
                                            "No argument provided".to_string(),
                                        ));
                                    }

                                    let flag_arg = args_iter.next().ok_or_else(|| {
                                        Error::InvalidFlagArgument(
                                            flag.to_string(),
                                            "No argument provided".to_string(),
                                        )
                                    })?;
                                    let parsed_arg =
                                        parse_string_argument(flag_arg, &flag_spec.arg_type)
                                            .map_err(|e| {
                                                Error::ArgumentParsing(
                                                    flag_arg.to_string(),
                                                    e.to_string(),
                                                )
                                            })?;
                                    self.parsed_flags
                                        .insert(flag_spec.name, Argument::Basic(parsed_arg));
                                }
                                ArgumentKind::List(_) | ArgumentKind::Enumeration(_) => {
                                    return Err(Error::Internal(
                                        "A list or enumeration is not a valid flag argument"
                                            .to_string(),
                                    ));
                                }
                                ArgumentKind::Flag => {
                                    self.parsed_flags.insert(flag_spec.name, Argument::Flag);
                                }
                            },
                            None => return Err(Error::UnknownFlag(flag.to_string())),
                        }
                    }
                }
                ArgKind::LonghandFlag => {
                    let flag_name = arg
                        .strip_prefix("--")
                        .expect("A longhand flag should start with --");
                    match self.flag_defs.get_flag_longhand(flag_name) {
                        Some(flag_spec) => match &flag_spec.arg_type {
                            ArgumentKind::Basic(_) => {
                                match args_iter.peek() {
                                    Some(arg) if arg.starts_with('-') => {
                                        return Err(Error::InvalidFlagArgument(
                                            flag_name.to_string(),
                                            "No argument provided".to_string(),
                                        ));
                                    }
                                    None => {
                                        return Err(Error::MissingCommandArgument(0));
                                    }
                                    _ => {}
                                }
                                let flag_arg = args_iter.next().ok_or_else(|| {
                                    Error::InvalidFlagArgument(
                                        flag_name.to_string(),
                                        "No argument provided".to_string(),
                                    )
                                })?;
                                let parsed_arg = parse_string_argument(
                                    flag_arg,
                                    &flag_spec.arg_type,
                                )
                                .map_err(|e| {
                                    Error::ArgumentParsing(flag_arg.to_string(), e.to_string())
                                })?;
                                self.parsed_flags
                                    .insert(flag_name, Argument::Basic(parsed_arg));
                            }
                            ArgumentKind::List(argument_kinds) => {
                                let mut list_args: Vec<BasicArgument> = Vec::new();
                                for (i, argument_kind) in argument_kinds.iter().enumerate() {
                                    let arg = args_iter
                                        .next()
                                        .ok_or(Error::MissingCommandArgument(i as u32))?;
                                    let argument = parse_string_argument(
                                        arg,
                                        &ArgumentKind::Basic(argument_kind.clone()),
                                    )
                                    .map_err(|e| {
                                        Error::ArgumentParsing(arg.to_string(), e.to_string())
                                    })?;
                                    list_args.push(argument);
                                }
                                self.parsed_flags
                                    .insert(flag_name, Argument::List(list_args));
                            }
                            ArgumentKind::Enumeration(argument_kind) => {
                                let mut list_args: Vec<BasicArgument> = Vec::new();
                                for arg in args_iter.by_ref() {
                                    let argument = parse_string_argument(
                                        arg,
                                        &ArgumentKind::Basic(argument_kind.clone()),
                                    )
                                    .map_err(|e| {
                                        Error::ArgumentParsing(arg.to_string(), e.to_string())
                                    })?;
                                    list_args.push(argument);
                                }
                                self.parsed_flags
                                    .insert(flag_name, Argument::List(list_args));
                            }
                            ArgumentKind::Flag => {
                                self.parsed_flags.insert(flag_name, Argument::Flag);
                            }
                        },
                        None => return Err(Error::UnknownFlag(flag_name.to_string())),
                    }
                }
                ArgKind::Argument => {
                    if let Some(arg_defs) = &self.arg_defs {
                        match arg_defs {
                            ArgumentKind::Basic(_) => match self.parsed_args {
                                Some(_) => {
                                    return Err(Error::TooManyArguments);
                                }
                                None => {
                                    let argument =
                                        parse_string_argument(arg, arg_defs).map_err(|e| {
                                            Error::ArgumentParsing(arg.to_string(), e.to_string())
                                        })?;
                                    self.parsed_args = Some(Argument::Basic(argument));
                                }
                            },
                            ArgumentKind::List(argument_kinds) => {
                                let mut list_args: Vec<BasicArgument> = Vec::new();
                                let mut arg_kinds = argument_kinds.iter().enumerate();
                                let argument = parse_string_argument(
                                    arg,
                                    &ArgumentKind::Basic(arg_kinds.next().unwrap().1.clone()),
                                )
                                .map_err(|e| {
                                    Error::ArgumentParsing(arg.to_string(), e.to_string())
                                })?;
                                list_args.push(argument);

                                for (i, argument_kind) in arg_kinds {
                                    let arg = args_iter
                                        .next()
                                        .ok_or(Error::MissingCommandArgument(i as u32))?;
                                    let argument = parse_string_argument(
                                        arg,
                                        &ArgumentKind::Basic(argument_kind.clone()),
                                    )
                                    .map_err(|e| {
                                        Error::ArgumentParsing(arg.to_string(), e.to_string())
                                    })?;
                                    list_args.push(argument);
                                }
                                self.parsed_args = Some(Argument::List(list_args));
                            }
                            ArgumentKind::Enumeration(argument_kind) => {
                                let mut list_args: Vec<BasicArgument> = Vec::new();
                                let argument = parse_string_argument(
                                    arg,
                                    &ArgumentKind::Basic(argument_kind.clone()),
                                )
                                .map_err(|e| {
                                    Error::ArgumentParsing(arg.to_string(), e.to_string())
                                })?;
                                list_args.push(argument);
                                for arg in args_iter.by_ref() {
                                    let argument = parse_string_argument(
                                        arg,
                                        &ArgumentKind::Basic(argument_kind.clone()),
                                    )
                                    .map_err(|e| {
                                        Error::ArgumentParsing(arg.to_string(), e.to_string())
                                    })?;
                                    list_args.push(argument);
                                }
                                self.parsed_args = Some(Argument::List(list_args));
                            }
                            ArgumentKind::Flag => {
                                panic!(
                                    "A flag argument should not be present in the argument list"
                                );
                            }
                        }
                    }
                    // TODO : If we didn't expect any arguments, what sanction should we give?
                    // Like `pwd` just ignores it so?
                }
            }
        }

        Ok(Command {
            command: *self.command,
            flags: self.parsed_flags,
            args: self.parsed_args,
        })
    }
}

enum ArgKind {
    ShorthandFlag,
    LonghandFlag,
    Argument,
}

fn arg_kind(arg: &str) -> ArgKind {
    if arg.starts_with("--") {
        ArgKind::LonghandFlag
    } else if arg.starts_with('-') && arg.len() > 1 {
        ArgKind::ShorthandFlag
    } else {
        ArgKind::Argument
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    #[error("Unknown flag: {0}")]
    UnknownFlag(String),
    #[error("Invalid flag argument: {0} - {1}")]
    InvalidFlagArgument(String, String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Argument parsing error: {0} - {1}")]
    ArgumentParsing(String, String),
    #[error("Missing command argument: {0}")]
    MissingCommandArgument(u32),
    #[error("Too many arguments")]
    TooManyArguments,
}
