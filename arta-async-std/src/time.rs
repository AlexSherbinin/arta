use crate::AsyncStdGlobalRuntime;
use arta::time::TimeRuntime;
use futures::prelude::Future;
use std::time::Duration;

impl TimeRuntime for AsyncStdGlobalRuntime {
    fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + Send {
        async_std::task::sleep(duration)
    }
}
