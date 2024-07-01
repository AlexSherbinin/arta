//! Green threads spawning and management.

use futures::Future;

/// Represents a handle to control spawned task execution.
pub trait RuntimeJoinHandle<T>: Future<Output = std::thread::Result<T>>
where
    T: Send + 'static,
{
    /// Cancel this task.
    fn cancel(self) -> impl Future<Output = Option<std::thread::Result<T>>> + Send;
}

/// Represents an async runtime that supports green threads spawning.
pub trait TaskRuntime: Send + Sync {
    /// Handle emitted on thread spawn.
    type JoinHandle<T>: RuntimeJoinHandle<T>
    where
        T: Send + 'static;

    /// Spawn thread.
    fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static;

    /// Spawn thread that may block on I/O operation or does some CPU heavy work.
    fn spawn_blocking<R>(&self, task: impl FnOnce() -> R + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static;
}
