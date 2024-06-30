use crate::fs::OsFile;
use futures::{AsyncRead, AsyncWrite, Future};
use std::{
    ffi::OsStr,
    path::Path,
    process::{ExitStatus, Output, Stdio},
};

pub trait RuntimeCommand {
    type Runtime: ProcessRuntime<Command = Self>;

    fn new(program: impl AsRef<OsStr>) -> Self
    where
        Self: Sized;

    fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self;

    fn args(&mut self, args: impl Iterator<Item = impl AsRef<OsStr>>) -> &mut Self;

    fn env(&mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> &mut Self;

    fn envs(
        &mut self,
        vars: impl Iterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
    ) -> &mut Self;

    fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self;

    fn env_clear(&mut self) -> &mut Self;

    fn current_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self;

    fn stdin(&mut self, stdin: impl Into<Stdio>) -> &mut Self;

    fn stdout(&mut self, stdout: impl Into<Stdio>) -> &mut Self;

    fn stderr(&mut self, stderr: impl Into<Stdio>) -> &mut Self;

    fn spawn(&mut self) -> std::io::Result<<Self::Runtime as ProcessRuntime>::Child>;

    fn output(&mut self) -> impl Future<Output = std::io::Result<Output>> + Send;

    fn status(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + Send;

    #[cfg(any(unix, doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    fn uid(&mut self, uid: u32) -> &mut Self;

    #[cfg(any(unix, doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    fn gid(&mut self, gid: u32) -> &mut Self;

    #[cfg(any(unix, doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    unsafe fn pre_exec(
        &mut self,
        f: impl FnMut() -> std::io::Result<()> + Send + Sync + 'static,
    ) -> &mut Self;

    #[cfg(any(windows, doc))]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    fn creation_flags(&mut self, flags: u32) -> &mut Self;
}

pub trait RuntimeChild {
    type Runtime: ProcessRuntime<Child = Self>;
    type Stdin<'a>: AsyncWrite + OsFile
    where
        Self: 'a;
    type Stdout<'a>: AsyncRead + OsFile
    where
        Self: 'a;
    type Stderr<'a>: AsyncRead + OsFile
    where
        Self: 'a;

    fn stdin(&mut self) -> Option<Self::Stdin<'_>>;

    fn stdout(&mut self) -> Option<Self::Stdout<'_>>;

    fn stderr(&mut self) -> Option<Self::Stderr<'_>>;

    fn id(&self) -> u32;
    fn kill(&mut self) -> std::io::Result<()>;
    fn output(self) -> impl Future<Output = std::io::Result<Output>> + Send;

    fn status(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + Send;
    fn try_status(&mut self) -> std::io::Result<Option<ExitStatus>>;
}

pub trait ProcessRuntime: Send + Sync {
    type Command: RuntimeCommand<Runtime = Self>;
    type Child: RuntimeChild<Runtime = Self>;
}
