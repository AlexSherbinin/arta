use std::time::Duration;

use futures::Future;

pub trait TimeRuntime {
    fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + Send;
}
