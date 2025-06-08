//! Errors that can be returned by commands.

pub type CommandError = String;

pub enum OperationResult {
    Success,
    Failure,
}
