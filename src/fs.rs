//! Filesystem manipulation operations.

mod dir_entry;
mod file;

pub use dir_entry::*;
pub use file::*;

use futures::{AsyncReadExt, AsyncWriteExt, Stream};
use std::{
    fs::{Metadata, OpenOptions, Permissions},
    future::Future,
    path::{Path, PathBuf},
    pin::pin,
};

/// Represents an async runtime that supports asynchronous access to filesystem.
pub trait FSRuntime: Send + Sync {
    /// Runtime's file.
    type File: RuntimeFile<Runtime = Self>;
    /// Runtime's dir entry.
    type DirEntry: RuntimeDirEntry;

    /// Returns the canonical, absolute form of a path with all intermediate
    /// components normalized and symbolic links resolved.
    ///
    /// This is an async version of [`std::fs::canonicalize`]
    fn canonicalize(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send;

    /// Copies the contents of one file to another. This function will also
    /// copy the permission bits of the original file to the destination file.
    ///
    /// This is an async version of [`std::fs::copy`]
    fn copy(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<u64>> + Send;

    /// Creates a new, empty directory at the provided path.
    ///
    /// This is an async version of [`std::fs::create_dir`]
    fn create_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Recursively create a directory and all of its parent components if they
    /// are missing.
    ///
    /// This is an async version of [`std::fs::create_dir_all`]
    fn create_dir_all(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Removes an empty directory.
    ///
    /// This is an async version of [`std::fs::remove_dir`]
    fn remove_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Returns an iterator over the entries within a directory.
    ///
    /// This is an async version of [`std::fs::remove_dir_all`]
    fn remove_dir_all(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Returns a stream over the entries within a directory.
    ///
    /// This is an async version of [`std::fs::read_dir`]
    fn read_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<
        Output = std::io::Result<impl Stream<Item = std::io::Result<Self::DirEntry>> + Send>,
    > + Send;

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This is an async version of [`std::fs::read_link`]
    fn read_link(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send;

    /// Creates a new symbolic link on the filesystem.
    ///
    /// This is an async version of [`std::os::unix::fs::symlink`]
    fn symlink(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Creates a new hard link on the filesystem.
    ///
    /// This is an async version of [`std::fs::hard_link`]
    fn hard_link(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Given a path, query the file system to get information about a file,
    /// directory, etc.
    ///
    /// This is an async version of [`std::fs::metadata`]
    fn metadata(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<Metadata>> + Send;

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This is an async version of [`std::fs::read`]
    fn read(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<Vec<u8>>> + Send {
        async {
            let mut file = pin!(Self::File::open(self, OpenOptions::new().read(true), path).await?);
            let file_size = file.metadata().await?.len() as usize;

            let mut buffer = Vec::with_capacity(file_size);
            file.read_to_end(&mut buffer).await?;

            Ok(buffer)
        }
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This is an async version of [`std::fs::read_to_string`]
    fn read_to_string(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<String>> + Send {
        async {
            let mut file = pin!(Self::File::open(self, OpenOptions::new().read(true), path).await?);
            let file_size = file.metadata().await?.len() as usize;

            let mut buffer = String::with_capacity(file_size);
            file.read_to_string(&mut buffer).await?;

            Ok(buffer)
        }
    }

    /// Write a slice as the entire contents of a file.
    ///
    /// This is an async version of [`std::fs::write`]
    fn write(
        &self,
        path: impl AsRef<Path> + Send,
        content: &[u8],
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        async {
            let mut file =
                pin!(Self::File::open(self, OpenOptions::new().create(true), path).await?);
            file.write_all(content).await
        }
    }

    /// Removes a file from the filesystem.
    ///
    /// This is an async version of [`std::fs::remove_file`]
    fn remove_file(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Rename a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This is an async version of [`std::fs::rename`]
    fn rename(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Changes the permissions found on a file or a directory.
    ///
    /// This is an async version of [`std::fs::set_permissions`]
    fn set_permissions(
        &self,
        path: impl AsRef<Path> + Send,
        permissions: Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Query the metadata about a file without following symlinks.
    ///
    /// This is an async version of [`std::fs::symlink_metadata`]
    fn symlink_metadata(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<Metadata>> + Send;
}
