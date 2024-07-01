mod child;
mod command;
mod outputs;

pub use child::*;
pub use command::*;
pub use outputs::*;

use crate::TokioGlobalRuntime;
use arta::process::ProcessRuntime;

impl ProcessRuntime for TokioGlobalRuntime {
    type Command = TokioCommand;
    type Child = TokioChild;
}
