use crate::AsyncStdGlobalRuntime;
use arta::task::{RuntimeJoinHandle, TaskRuntime};
use futures::{prelude::Future, FutureExt};
use std::{
    panic::AssertUnwindSafe,
    pin::Pin,
    task::{Context, Poll},
};

pub struct AsyncStdJoinHandle<T> {
    inner: async_std::task::JoinHandle<std::thread::Result<T>>,
}

impl<T> Future for AsyncStdJoinHandle<T>
where
    T: Send,
{
    type Output = std::thread::Result<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

impl<T> RuntimeJoinHandle<T> for AsyncStdJoinHandle<T>
where
    T: Send + 'static,
{
    fn cancel(self) -> impl Future<Output = Option<std::thread::Result<T>>> + Send {
        self.inner.cancel()
    }
}

impl TaskRuntime for AsyncStdGlobalRuntime {
    type JoinHandle<T> = AsyncStdJoinHandle<T> where T: Send + 'static;

    fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static,
    {
        AsyncStdJoinHandle {
            inner: async_std::task::spawn(AssertUnwindSafe(future).catch_unwind()),
        }
    }

    fn spawn_blocking<R>(&self, task: impl FnOnce() -> R + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static,
    {
        AsyncStdJoinHandle {
            inner: async_std::task::spawn_blocking(|| {
                std::panic::catch_unwind(AssertUnwindSafe(task))
            }),
        }
    }
}
