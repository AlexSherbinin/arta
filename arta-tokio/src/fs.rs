//! Tokio specific filesystem manipulation implementations.
mod dir_entry;
mod file;

pub use dir_entry::*;
pub use file::*;

use crate::TokioGlobalRuntime;
use arta::fs::FSRuntime;
use futures::prelude::Stream;
use std::{
    fs::Metadata,
    future::Future,
    path::{Path, PathBuf},
    task::{ready, Poll},
};

impl FSRuntime for TokioGlobalRuntime {
    type File = TokioFile;
    type DirEntry = TokioDirEntry;

    fn canonicalize(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send {
        tokio::fs::canonicalize(path)
    }

    fn copy(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<u64>> + Send {
        tokio::fs::copy(from, to)
    }

    fn create_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::create_dir(path)
    }

    fn create_dir_all(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::create_dir_all(path)
    }

    fn remove_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::remove_dir(path)
    }

    fn remove_dir_all(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::remove_dir_all(path)
    }

    async fn read_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> std::io::Result<impl Stream<Item = std::io::Result<Self::DirEntry>> + Send> {
        let mut read_dir = tokio::fs::read_dir(path).await?;
        let read_dir = futures::stream::poll_fn(move |cx| {
            let entry = ready!(read_dir.poll_next_entry(cx));
            let entry = match entry {
                Ok(entry) => entry.map(|entry| Ok(TokioDirEntry { inner: entry })),
                Err(err) => Some(Err(err)),
            };

            Poll::Ready(entry)
        });

        Ok(read_dir)
    }

    fn read_link(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send {
        tokio::fs::read_link(path)
    }

    #[cfg(unix)]
    fn symlink(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::symlink(from, to)
    }

    #[cfg(windows)]
    fn symlink_dir(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::symlink_dir(from, to)
    }

    #[cfg(windows)]
    fn symlink_file(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::symlink_file(from, to)
    }

    fn hard_link(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::hard_link(from, to)
    }

    fn metadata(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<Metadata>> + Send {
        tokio::fs::metadata(path)
    }

    fn remove_file(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::remove_file(path)
    }

    fn rename(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::rename(from, to)
    }

    fn set_permissions(
        &self,
        path: impl AsRef<Path> + Send,
        permissions: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        tokio::fs::set_permissions(path, permissions)
    }

    fn symlink_metadata(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<Metadata>> + Send {
        tokio::fs::symlink_metadata(path)
    }
}
