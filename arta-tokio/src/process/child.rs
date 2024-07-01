use super::{TokioStderr, TokioStdin, TokioStdout};
use crate::TokioGlobalRuntime;
use arta::process::RuntimeChild;
use std::{
    future::Future,
    process::{ExitStatus, Output},
};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

/// Tokio specific [`RuntimeChild`] implementation.
pub struct TokioChild {
    pub(super) id: u32,
    pub(super) inner: tokio::process::Child,
}

impl RuntimeChild for TokioChild {
    type Runtime = TokioGlobalRuntime;
    type Stdin<'a> = TokioStdin<'a>;
    type Stdout<'a> = TokioStdout<'a>;
    type Stderr<'a> = TokioStderr<'a>;

    fn stdin(&mut self) -> Option<Self::Stdin<'_>> {
        self.inner.stdin.as_mut().map(|stdin| TokioStdin {
            inner: stdin.compat_write(),
        })
    }

    fn stdout(&mut self) -> Option<Self::Stdout<'_>> {
        self.inner.stdout.as_mut().map(|stdout| TokioStdout {
            inner: stdout.compat(),
        })
    }

    fn stderr(&mut self) -> Option<Self::Stderr<'_>> {
        self.inner.stderr.as_mut().map(|stderr| TokioStderr {
            inner: stderr.compat(),
        })
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn kill(&mut self) -> std::io::Result<()> {
        self.inner.start_kill()
    }

    fn output(self) -> impl Future<Output = std::io::Result<Output>> + Send {
        self.inner.wait_with_output()
    }

    fn status(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + Send {
        self.inner.wait()
    }

    fn try_status(&mut self) -> std::io::Result<Option<ExitStatus>> {
        self.inner.try_wait()
    }
}
