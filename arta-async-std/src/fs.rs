mod dir_entry;
mod file;

pub use dir_entry::*;
pub use file::*;

use crate::AsyncStdGlobalRuntime;
use arta::fs::FSRuntime;
use async_std::stream::StreamExt;
use futures::{prelude::Future, TryFutureExt};
use std::path::PathBuf;

impl FSRuntime for AsyncStdGlobalRuntime {
    type File = AsyncStdFile;
    type DirEntry = AsyncStdDirEntry;

    fn canonicalize(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::canonicalize(path).map_ok(Into::into)
    }

    fn copy(
        &self,
        from: impl AsRef<std::path::Path> + Send,
        to: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<u64>> + Send {
        let from: async_std::path::PathBuf = from.as_ref().to_owned().into();
        let to: async_std::path::PathBuf = to.as_ref().to_owned().into();

        async_std::fs::copy(from, to)
    }

    fn create_dir(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::create_dir(path)
    }

    fn create_dir_all(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::create_dir_all(path)
    }

    fn remove_dir(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::remove_dir(path)
    }

    fn remove_dir_all(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::remove_dir_all(path)
    }

    fn read_dir(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<
        Output = std::io::Result<
            impl futures::prelude::Stream<Item = std::io::Result<Self::DirEntry>> + Send,
        >,
    > + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::read_dir(path).map_ok(|stream| {
            stream.map(|entry| entry.map(|entry| AsyncStdDirEntry { inner: entry }))
        })
    }

    fn read_link(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::read_link(path).map_ok(Into::into)
    }

    #[cfg(unix)]
    fn symlink(
        &self,
        from: impl AsRef<std::path::Path> + Send,
        to: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let from: async_std::path::PathBuf = from.as_ref().to_owned().into();
        let to: async_std::path::PathBuf = to.as_ref().to_owned().into();

        async_std::os::unix::fs::symlink(from, to)
    }

    #[cfg(windows)]
    fn symlink_dir(
        &self,
        from: impl AsRef<std::path::Path> + Send,
        to: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let from: async_std::path::PathBuf = from.as_ref().to_owned().into();
        let to: async_std::path::PathBuf = to.as_ref().to_owned().into();

        async_std::os::windows::fs::symlink_dir(from, to)
    }

    #[cfg(windows)]
    fn symlink_file(
        &self,
        from: impl AsRef<std::path::Path> + Send,
        to: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let from: async_std::path::PathBuf = from.as_ref().to_owned().into();
        let to: async_std::path::PathBuf = to.as_ref().to_owned().into();

        async_std::os::windows::fs::symlink_file(from, to)
    }

    fn hard_link(
        &self,
        from: impl AsRef<std::path::Path> + Send,
        to: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let from: async_std::path::PathBuf = from.as_ref().to_owned().into();
        let to: async_std::path::PathBuf = to.as_ref().to_owned().into();

        async_std::fs::hard_link(from, to)
    }

    fn metadata(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<std::fs::Metadata>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::metadata(path)
    }

    fn remove_file(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::remove_file(path)
    }

    fn rename(
        &self,
        from: impl AsRef<std::path::Path> + Send,
        to: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let from: async_std::path::PathBuf = from.as_ref().to_owned().into();
        let to: async_std::path::PathBuf = to.as_ref().to_owned().into();

        async_std::fs::rename(from, to)
    }

    fn set_permissions(
        &self,
        path: impl AsRef<std::path::Path> + Send,
        permissions: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::set_permissions(path, permissions)
    }

    fn symlink_metadata(
        &self,
        path: impl AsRef<std::path::Path> + Send,
    ) -> impl Future<Output = std::io::Result<std::fs::Metadata>> + Send {
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();
        async_std::fs::symlink_metadata(path)
    }
}
