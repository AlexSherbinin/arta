use crate::runtimes::TokioGlobalRuntime;
use arta::time::TimeRuntime;
use std::{future::Future, time::Duration};

impl TimeRuntime for TokioGlobalRuntime {
    fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + Send {
        tokio::time::sleep(duration)
    }
}
