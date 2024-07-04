use std::{
    ffi::OsStr,
    path::Path,
    process::{ExitStatus, Output, Stdio},
};

use arta::process::{ProcessRuntime, RuntimeCommand};
use futures::prelude::Future;

use crate::AsyncStdGlobalRuntime;

use super::AsyncStdChild;

pub struct AsyncStdCommand {
    inner: async_std::process::Command,
}

impl RuntimeCommand for AsyncStdCommand {
    type Runtime = AsyncStdGlobalRuntime;

    fn new(program: impl AsRef<OsStr>) -> Self
    where
        Self: Sized,
    {
        Self {
            inner: async_std::process::Command::new(program),
        }
    }

    fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.inner.arg(arg);
        self
    }

    fn args(&mut self, args: impl Iterator<Item = impl AsRef<OsStr>>) -> &mut Self {
        self.inner.args(args);
        self
    }

    fn env(&mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> &mut Self {
        self.inner.env(key, value);
        self
    }

    fn envs(
        &mut self,
        vars: impl Iterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
    ) -> &mut Self {
        self.inner.envs(vars);
        self
    }

    fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self {
        self.inner.env_remove(key);
        self
    }

    fn env_clear(&mut self) -> &mut Self {
        self.inner.env_clear();
        self
    }

    fn current_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.inner.current_dir(dir);
        self
    }

    fn stdin(&mut self, stdin: impl Into<Stdio>) -> &mut Self {
        self.inner.stdin(stdin);
        self
    }

    fn stdout(&mut self, stdout: impl Into<Stdio>) -> &mut Self {
        self.inner.stdout(stdout);
        self
    }

    fn stderr(&mut self, stderr: impl Into<Stdio>) -> &mut Self {
        self.inner.stderr(stderr);
        self
    }

    fn spawn(&mut self) -> std::io::Result<<Self::Runtime as ProcessRuntime>::Child> {
        self.inner
            .spawn()
            .map(|child| AsyncStdChild { inner: child })
    }

    fn output(&mut self) -> impl Future<Output = std::io::Result<Output>> + Send {
        self.inner.output()
    }

    fn status(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + Send {
        self.inner.status()
    }

    #[cfg(unix)]
    fn uid(&mut self, uid: u32) -> &mut Self {
        use async_std::os::unix::process::CommandExt;

        self.inner.uid(uid);
        self
    }

    #[cfg(unix)]
    fn gid(&mut self, gid: u32) -> &mut Self {
        use async_std::os::unix::process::CommandExt;

        self.inner.gid(gid);
        self
    }

    #[cfg(unix)]
    unsafe fn pre_exec(
        &mut self,
        f: impl FnMut() -> std::io::Result<()> + Send + Sync + 'static,
    ) -> &mut Self {
        use async_std::os::unix::process::CommandExt;

        self.inner.pre_exec(f);
        self
    }

    #[cfg(windows)]
    fn creation_flags(&mut self, flags: u32) -> &mut Self {
        use async_std::os::windows::process::CommandExt;

        self.inner.creation_flags(flags);
        self
    }
}