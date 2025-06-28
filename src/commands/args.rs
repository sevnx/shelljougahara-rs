//! The supproted argument types for a command

use strum_macros::Display;

/// Possible arguments of a command or flag, with their values
#[derive(Debug, PartialEq)]
pub enum Argument {
    Basic(BasicArgument),
    List(Vec<BasicArgument>),
    Flag,
}

/// The type of basic single argument
#[derive(Debug, PartialEq)]
pub enum BasicArgument {
    String(String),
    Integer(i64),
}

/// The type of an argument, used for definitions and parsing
#[derive(Eq, Hash, PartialEq, Display)]
pub enum ArgumentKind {
    /// A simple string argument, also used for paths
    /// Example - argument to echo : `echo "Hello, world!"`
    Basic(BasicArgumentKind),
    /// A list of arguments of different types
    /// Example - arguments to chmod : `chmod 755 file.txt`
    #[allow(unused)]
    List(Vec<BasicArgumentKind>),
    /// An enumeration of arguments
    /// Example - argument to rm : `rm file1.txt file2.txt`
    Enumeration(BasicArgumentKind),
    /// A flag argument
    /// Example - flag to rm : `rm -f`
    Flag,
}

/// The type of basic single argument
#[derive(Eq, Hash, PartialEq, Display, Clone)]
pub enum BasicArgumentKind {
    String,
    Integer,
}

pub fn parse_string_argument(arg: &str, kind: &ArgumentKind) -> Result<BasicArgument, String> {
    match kind {
        ArgumentKind::Basic(kind) => match kind {
            BasicArgumentKind::String => Ok(BasicArgument::String(arg.to_string())),
            BasicArgumentKind::Integer => {
                let arg = arg.parse::<i64>().map_err(|e| e.to_string())?;
                Ok(BasicArgument::Integer(arg))
            }
        },
        argument => Err(format!("Invalid argument kind: {argument}")),
    }
}
