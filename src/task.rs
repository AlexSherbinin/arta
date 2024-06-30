use futures::Future;

pub trait RuntimeJoinHandle<T>: Future<Output = std::thread::Result<T>>
where
    T: Send + 'static,
{
    fn cancel(self) -> impl Future<Output = Option<std::thread::Result<T>>> + Send;
}

pub trait TaskRuntime: Send + Sync {
    type JoinHandle<T>: RuntimeJoinHandle<T>
    where
        T: Send + 'static;

    fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static;

    fn spawn_blocking<R>(&self, task: impl FnOnce() -> R + Send + 'static) -> Self::JoinHandle<R>
    where
        R: Send + 'static;
}
