use super::{AsyncStdStderr, AsyncStdStdin, AsyncStdStdout};
use crate::AsyncStdGlobalRuntime;
use arta::process::RuntimeChild;
use futures::prelude::Future;
use std::process::{ExitStatus, Output};

/// Async-std specific [`RuntimeChild`] implementation.
pub struct AsyncStdChild {
    pub(super) inner: async_std::process::Child,
}

impl RuntimeChild for AsyncStdChild {
    type Runtime = AsyncStdGlobalRuntime;

    type Stdin<'a> = AsyncStdStdin<'a>
    where
        Self: 'a;

    type Stdout<'a> = AsyncStdStdout<'a>
    where
        Self: 'a;

    type Stderr<'a> = AsyncStdStderr<'a>
    where
        Self: 'a;

    fn stdin(&mut self) -> Option<Self::Stdin<'_>> {
        self.inner
            .stdin
            .as_mut()
            .map(|stdin| AsyncStdStdin { inner: stdin })
    }

    fn stdout(&mut self) -> Option<Self::Stdout<'_>> {
        self.inner
            .stdout
            .as_mut()
            .map(|stdout| AsyncStdStdout { inner: stdout })
    }

    fn stderr(&mut self) -> Option<Self::Stderr<'_>> {
        self.inner
            .stderr
            .as_mut()
            .map(|stderr| AsyncStdStderr { inner: stderr })
    }

    fn id(&self) -> u32 {
        self.inner.id()
    }

    fn kill(&mut self) -> std::io::Result<()> {
        self.inner.kill()
    }

    fn output(self) -> impl Future<Output = std::io::Result<Output>> + Send {
        self.inner.output()
    }

    fn status(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + Send {
        self.inner.status()
    }

    fn try_status(&mut self) -> std::io::Result<Option<ExitStatus>> {
        self.inner.try_status()
    }
}
