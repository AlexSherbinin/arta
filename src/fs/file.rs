use cfg_if::cfg_if;
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use std::{
    fs::{Metadata, OpenOptions, Permissions},
    future::Future,
    path::Path,
};

use super::FSRuntime;

cfg_if! {
    if #[cfg(windows)] {
        /// Represents a file accessor that implements OS specific methods.
        pub trait OsFile: std::os::windows::io::AsRawHandle + std::os::windows::io::AsHandle {}
        impl<T> OsFile for T where std::os::windows::io::AsRawHandle + std::os::windows::io::AsHandle {}
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        /// Represents a file accessor that implements OS specific methods.
        pub trait OsFile: std::os::fd::AsRawFd + std::os::fd::AsFd {}
        impl<T> OsFile for T where T: std::os::fd::AsRawFd + std::os::fd::AsFd {}
    } else {
        /// Represents a file accessor that implements OS specific methods.
        pub trait OsFile {}
        impl<T> OsFile for T {}
    }
}

cfg_if! {
    if #[cfg(windows)] {
        /// Represents a file that can be constructed from OS specific handle.
        pub trait FromOsOwnedDescriptor: From<std::os::windows::io::OwnedHandle> {}
        impl<T> FromOsOwnedDescriptor for T where T: From<std::os::windows::io::OwnedHandle> {}
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        /// Represents a file that can be constructed from OS specific handle.
        pub trait FromOsOnwedDescriptor: From<std::os::fd::OwnedFd> {}
        impl<T> FromOsOnwedDescriptor for T where T: From<std::os::fd::OwnedFd> {}
    } else {
        /// Represents a file that can be constructed from OS specific handle.
        pub trait FromOsOwnedDescriptor {}
        impl<T> FromOsOwnedDescriptor for T {}
    }
}

/// Represents an object providing access to an open file on the filesystem.
///
/// An async version of [`std::fs::File`].
pub trait RuntimeFile:
    OsFile + FromOsOnwedDescriptor + AsyncRead + AsyncWrite + AsyncSeek + Send + Sync
{
    /// An async runtime.
    type Runtime: FSRuntime<File = Self>;

    /// Attempts to open a file with specified `OpenOptions`.
    ///
    /// An async version of [`std::fs::File::open`] but also consumes an `OpenOptions` instead of
    /// opening in read-only mode.
    fn open(
        runtime: &Self::Runtime,
        open_options: &OpenOptions,
        path: impl AsRef<Path>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;

    /// Truncates or extends the underlying file, updating the size of
    /// this file to become `size`.
    ///
    /// An async version of [`std::fs::File::set_len`].
    fn set_len(&self, size: u64) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Queries metadata about the underlying file.
    ///
    /// An async version of [`std::fs::File::metadata`].
    fn metadata(&self) -> impl Future<Output = std::io::Result<Metadata>> + Send;

    /// Attempts to sync all OS-internal file content and metadata to disk.
    ///
    /// An async version of [`std::fs::File::sync_all`].
    fn sync_all(&self) -> impl Future<Output = std::io::Result<()>> + Send;

    /// This function is similar to [`RuntimeFile::sync_all`], except that it might not
    /// synchronize file metadata to the filesystem.
    ///
    /// An async version of [`std::fs::File::sync_data`].
    fn sync_data(&self) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Changes the permissions on the underlying file.
    ///
    /// An async version of [`std::fs::File::set_permissions`].
    fn set_permissions(
        &self,
        permissions: Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}
