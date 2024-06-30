use crate::runtimes::TokioGlobalRuntime;
use arta::task::{RuntimeJoinHandle, TaskRuntime};
use futures::{prelude::Future, FutureExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct TokioJoinHandle<T> {
    inner: tokio::task::JoinHandle<T>,
}

impl<T> Future for TokioJoinHandle<T> {
    type Output = std::thread::Result<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // There's no UB because we are consuming owned handle on cancel method.
        self.inner
            .poll_unpin(cx)
            .map_err(|err| unsafe { err.try_into_panic().unwrap_unchecked() })
    }
}

struct CancelFuture<T> {
    handle: TokioJoinHandle<T>,
}

impl<T> Future for CancelFuture<T> {
    type Output = Option<std::thread::Result<T>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = if let Poll::Ready(result) = self.handle.poll_unpin(cx) {
            Some(result)
        } else {
            self.handle.inner.abort();
            None
        };

        Poll::Ready(result)
    }
}

impl<T> RuntimeJoinHandle<T> for TokioJoinHandle<T>
where
    T: Send + 'static,
{
    fn cancel(self) -> impl Future<Output = Option<std::thread::Result<T>>> + Send {
        CancelFuture { handle: self }
    }
}

impl TaskRuntime for TokioGlobalRuntime {
    type JoinHandle<T> = TokioJoinHandle<T> where T: Send + 'static;

    fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static,
    {
        TokioJoinHandle {
            inner: tokio::task::spawn(future),
        }
    }

    fn spawn_blocking<R>(&self, task: impl FnOnce() -> R + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static,
    {
        TokioJoinHandle {
            inner: tokio::task::spawn_blocking(task),
        }
    }
}
