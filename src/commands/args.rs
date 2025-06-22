//! The supproted argument types for a command

use strum_macros::Display;

/// Possible arguments of a command or flag, with their values
#[derive(Debug, PartialEq)]
pub enum Argument {
    String(String),
    Integer(i64),
    List(Vec<Argument>),
    Flag,
}

/// The type of an argument, used for definitions and parsing
#[derive(Eq, Hash, PartialEq, Display)]
pub enum ArgumentKind {
    /// A simple string argument, also used for paths
    /// Example - argument to echo : `echo "Hello, world!"`
    String,
    /// A simple integer argument
    /// Example - argument to head : `head -n 10 file.txt`
    Integer,
    /// A list of arguments of different types
    /// Example - arguments to chmod : `chmod 755 file.txt`
    List(Vec<ArgumentKind>),
    /// An enumeration of arguments
    /// Example - argument to rm : `rm file1.txt file2.txt`
    Enumeration(Box<ArgumentKind>),
    /// A flag argument
    /// Example - flag to rm : `rm -f`
    Flag,
}

pub fn parse_string_argument(arg: &str, kind: &ArgumentKind) -> Result<Argument, String> {
    match kind {
        ArgumentKind::String => Ok(Argument::String(arg.to_string())),
        ArgumentKind::Integer => {
            let arg = arg.parse::<i64>().map_err(|e| e.to_string())?;
            Ok(Argument::Integer(arg))
        }
        _ => Err(format!("Invalid argument kind: {kind}")),
    }
}
