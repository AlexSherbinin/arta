//! Process creation and management operations.

use crate::fs::OsFile;
use futures::{AsyncRead, AsyncWrite, Future};
use std::{
    ffi::OsStr,
    path::Path,
    process::{ExitStatus, Output, Stdio},
};

/// Process builder. An async version of [`std::process::Command`].
pub trait RuntimeCommand {
    /// An async runtime.
    type Runtime: ProcessRuntime<Command = Self>;

    /// Constructs a new `RuntimeCommand` for launching the program at
    /// path `program`, with the following default configuration:
    ///
    /// * No arguments to the program
    /// * Inherit the current process's environment
    /// * Inherit the current process's working directory
    /// * Inherit stdin/stdout/stderr for [`spawn`] or [`status`], but create pipes for [`output`]
    ///
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    ///
    /// Builder methods are provided to change these defaults and
    /// otherwise configure the process.
    ///
    /// If `program` is not an absolute path, the `PATH` will be searched in
    /// an OS-defined way.
    ///
    /// The search path to be used may be controlled by setting the
    /// `PATH` environment variable on the Command,
    /// but this has some implementation limitations on Windows
    /// (see issue #37519).
    fn new(program: impl AsRef<OsStr>) -> Self
    where
        Self: Sized;

    /// Adds an argument to pass to the program.
    fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self;

    /// Adds multiple arguments to pass to the program.
    fn args(&mut self, args: impl Iterator<Item = impl AsRef<OsStr>>) -> &mut Self;

    /// Inserts or updates multiple explicit environment variable mappings.
    ///
    /// Child processes will inherit environment variables from their parent process by default.
    /// Environment variables explicitly set using [`RuntimeCommand::envs`] take precedence over inherited
    /// variables. You can disable environment variable inheritance entirely using
    /// [`RuntimeCommand::env_clear`] or for a single key using [`RuntimeCommand::env_remove`].
    ///
    /// Note that environment variable names are case-insensitive (but case-preserving) on Windows
    /// and case-sensitive on all other platforms.
    fn envs(
        &mut self,
        vars: impl Iterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
    ) -> &mut Self;

    /// Removes an explicitly set environment variable and prevents inheriting it from a parent
    /// process.
    ///
    /// This method will remove the explicit value of an environment variable set via
    /// [`RuntimeCommand::env`] or [`RuntimeCommand::envs`]. In addition, it will prevent the spawned child
    /// process from inheriting that environment variable from its parent process.
    ///
    /// To clear all explicitly set environment variables and disable all environment variable
    /// inheritance, you can use [`RuntimeCommand::env_clear`].
    fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self;

    /// Clears all explicitly set environment variables and prevents inheriting any parent process
    /// environment variables.
    ///
    /// This method will remove all explicitly added environment variables set via [`RuntimeCommand::env`]
    /// or [`RuntimeCommand::envs`]. In addition, it will prevent the spawned child process from inheriting
    /// any environment variable from its parent process.
    fn env_clear(&mut self) -> &mut Self;

    /// Sets the working directory for the child process.
    ///
    /// # Platform-specific behavior
    ///
    /// If the program path is relative (e.g., `"./script.sh"`), it's ambiguous
    /// whether it should be interpreted relative to the parent's working
    /// directory or relative to `current_dir`. The behavior in this case is
    /// platform specific and unstable, and it's recommended to use
    /// [`canonicalize`] to get an absolute program path instead.
    ///
    /// [`canonicalize`]: crate::fs::FSRuntime::canonicalize
    fn current_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self;

    /// Configuration for the child process's standard input (stdin) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    fn stdin(&mut self, stdin: impl Into<Stdio>) -> &mut Self;

    /// Configuration for the child process's standard output (stdout) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    fn stdout(&mut self, stdout: impl Into<Stdio>) -> &mut Self;

    /// Configuration for the child process's standard error (stderr) handle.
    ///
    /// Defaults to [`inherit`] when used with [`spawn`] or [`status`], and
    /// defaults to [`piped`] when used with [`output`].
    ///
    /// [`inherit`]: Stdio::inherit
    /// [`piped`]: Stdio::piped
    /// [`spawn`]: Self::spawn
    /// [`status`]: Self::status
    /// [`output`]: Self::output
    fn stderr(&mut self, stderr: impl Into<Stdio>) -> &mut Self;

    /// Executes the command as a child process, returning a handle to it.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    fn spawn(&mut self) -> std::io::Result<<Self::Runtime as ProcessRuntime>::Child>;

    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the
    /// resulting output). Stdin is not inherited from the parent and any
    /// attempt by the child process to read from the stdin stream will result
    /// in the stream immediately closing.
    ///
    /// An async version of [`std::process::Command::output`].
    fn output(&mut self) -> impl Future<Output = std::io::Result<Output>> + Send;

    /// Executes a command as a child process, waiting for it to finish and
    /// collecting its status.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// An async version of [`std::process::Command::status`].
    fn status(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + Send;

    /// Sets the child process's user ID. This translates to a
    /// `setuid` call in the child process. Failure in the `setuid`
    /// call will cause the spawn to fail.
    ///
    /// # Notes
    ///
    /// This will also trigger a call to `setgroups(0, NULL)` in the child
    /// process if no groups have been specified.
    /// This removes supplementary groups that might have given the child
    /// unwanted permissions.
    #[cfg(any(unix, doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    fn uid(&mut self, uid: u32) -> &mut Self;

    /// Similar to `uid`, but sets the group ID of the child process. This has
    /// the same semantics as the `uid` field.
    #[cfg(any(unix, doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    fn gid(&mut self, gid: u32) -> &mut Self;

    /// Schedules a closure to be run just before the `exec` function is
    /// invoked.
    ///
    /// The closure is allowed to return an I/O error whose OS error code will
    /// be communicated back to the parent and returned as an error from when
    /// the spawn was requested.
    ///
    /// Multiple closures can be registered and they will be called in order of
    /// their registration. If a closure returns `Err` then no further closures
    /// will be called and the spawn operation will immediately return with a
    /// failure.
    ///
    /// # Notes and Safety
    ///
    /// This closure will be run in the context of the child process after a
    /// `fork`. This primarily means that any modifications made to memory on
    /// behalf of this closure will **not** be visible to the parent process.
    /// This is often a very constrained environment where normal operations
    /// like `malloc`, accessing environment variables through [`mod@std::env`]
    /// or acquiring a mutex are not guaranteed to work (due to
    /// other threads perhaps still running when the `fork` was run).
    ///
    /// For further details refer to the [POSIX fork() specification]
    /// and the equivalent documentation for any targeted
    /// platform, especially the requirements around *async-signal-safety*.
    ///
    /// This also means that all resources such as file descriptors and
    /// memory-mapped regions got duplicated. It is your responsibility to make
    /// sure that the closure does not violate library invariants by making
    /// invalid use of these duplicates.
    ///
    /// Panicking in the closure is safe only if all the format arguments for the
    /// panic message can be safely formatted; this is because although
    /// `Command` calls [`std::panic::always_abort`]
    /// before calling the pre_exec hook, panic will still try to format the
    /// panic message.
    ///
    /// When this closure is run, aspects such as the stdio file descriptors and
    /// working directory have successfully been changed, so output to these
    /// locations might not appear where intended.
    ///
    /// [POSIX fork() specification]:
    ///     https://pubs.opengroup.org/onlinepubs/9699919799/functions/fork.html
    #[allow(clippy::missing_safety_doc)]
    #[cfg(any(unix, doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    unsafe fn pre_exec(
        &mut self,
        f: impl FnMut() -> std::io::Result<()> + Send + Sync + 'static,
    ) -> &mut Self;

    /// Sets the [process creation flags][1] to be passed to `CreateProcess`.
    ///
    /// These will always be ORed with `CREATE_UNICODE_ENVIRONMENT`.
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    #[cfg(any(windows, doc))]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    fn creation_flags(&mut self, flags: u32) -> &mut Self;
}

/// Representation of a running or exited child process. An async version of [`std::process::Child`].
pub trait RuntimeChild {
    /// An async runtime.
    type Runtime: ProcessRuntime<Child = Self>;
    /// Represents a stdin implementation specific to runtime.
    type Stdin<'a>: AsyncWrite + OsFile
    where
        Self: 'a;

    /// Represents a stdout implementation specific to runtime.
    type Stdout<'a>: AsyncRead + OsFile
    where
        Self: 'a;

    /// Represents a stderr implementation specific to runtime.
    type Stderr<'a>: AsyncRead + OsFile
    where
        Self: 'a;

    /// Returns child's stdin.
    fn stdin(&mut self) -> Option<Self::Stdin<'_>>;

    /// Returns child's stdout.
    fn stdout(&mut self) -> Option<Self::Stdout<'_>>;

    /// Returns child's stderr.
    fn stderr(&mut self) -> Option<Self::Stderr<'_>>;

    /// Returns the OS-assigned process identifier associated with this child.
    fn id(&self) -> u32;

    /// Forces the child process to exit. If the child has already exited, `Ok(())`
    /// is returned.
    fn kill(&mut self) -> std::io::Result<()>;

    /// Simultaneously waits for the child to exit and collect all remaining
    /// output on the stdout/stderr handles, returning an `Output`
    /// instance.
    fn output(self) -> impl Future<Output = std::io::Result<Output>> + Send;

    /// Waits for the child to exit completely, returning the status that it
    /// exited with. This function will continue to have the same return value
    /// after it has been called at least once.
    ///
    /// An async version of [`std::process::Child::wait`].
    fn status(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + Send;

    /// Attempts to collect the exit status of the child if it has already
    /// exited.
    ///
    /// This function will not block the calling thread and will only
    /// check to see if the child process has exited or not. If the child has
    /// exited then on Unix the process ID is reaped. This function is
    /// guaranteed to repeatedly return a successful exit status so long as the
    /// child has already exited.
    fn try_status(&mut self) -> std::io::Result<Option<ExitStatus>>;
}

/// Represents an async runtime that supports asynchronous process management.
pub trait ProcessRuntime: Send + Sync {
    /// Process builder for runtime. An async version of [`std::process::Command`].
    type Command: RuntimeCommand<Runtime = Self>;
    /// Runtime's process child. An async version of [`std::process::Child`].
    type Child: RuntimeChild<Runtime = Self>;
}
