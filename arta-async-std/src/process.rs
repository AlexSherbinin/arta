mod child;
mod command;
mod outputs;

use arta::process::ProcessRuntime;
pub use child::*;
pub use command::*;
pub use outputs::*;

use crate::AsyncStdGlobalRuntime;

impl ProcessRuntime for AsyncStdGlobalRuntime {
    type Command = AsyncStdCommand;
    type Child = AsyncStdChild;
}
