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

pub trait FSRuntime: Send + Sync {
    type File: RuntimeFile<Runtime = Self>;
    type DirEntry: RuntimeDirEntry;

    fn canonicalize(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send;

    fn copy(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<u64>> + Send;

    fn create_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn create_dir_all(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn remove_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn remove_dir_all(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn read_dir(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<
        Output = std::io::Result<impl Stream<Item = std::io::Result<Self::DirEntry>> + Send>,
    > + Send;

    fn read_link(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<PathBuf>> + Send;

    fn symlink(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn hard_link(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn metadata(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<Metadata>> + Send;

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

    fn remove_file(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn rename(
        &self,
        from: impl AsRef<Path> + Send,
        to: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn set_permissions(
        &self,
        path: impl AsRef<Path> + Send,
        permissions: Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn symlink_metadata(
        &self,
        path: impl AsRef<Path> + Send,
    ) -> impl Future<Output = std::io::Result<Metadata>> + Send;
}
