//! Time tracking operations.

use futures::Future;
use std::time::Duration;

/// Represents an async runtime that supports asynchronous timer.
pub trait TimeRuntime {
    /// Sleep for a specified time.
    fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + Send;
}
