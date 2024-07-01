use crate::TokioGlobalRuntime;
use arta::time::TimeRuntime;
use std::{future::Future, time::Duration};

#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
impl TimeRuntime for TokioGlobalRuntime {
    fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + Send {
        tokio::time::sleep(duration)
    }
}
