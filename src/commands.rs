//! Commands, the things that the shell can execute.

use std::io::Write;

use crate::Shell;
use flags::Flags;
use list::CommandList;

mod flags;
mod list;

#[enum_dispatch::enum_dispatch(CommandList)]
pub trait Command {
    fn name(&self) -> &str;
    fn flags(&self) -> Flags;
    fn execute(
        &self,
        args: &[String],
        shell: &mut Shell,
        writer: &mut dyn Write,
    ) -> Result<(), String>;
}
