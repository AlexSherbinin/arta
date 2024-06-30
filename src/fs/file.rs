use cfg_if::cfg_if;
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use std::{
    fs::{Metadata, OpenOptions, Permissions},
    future::Future,
    path::Path,
};

cfg_if! {
    if #[cfg(windows)] {
        pub trait OsFile: std::os::windows::io::AsRawHandle + std::os::windows::io::AsHandle {}
        impl<T> OsFile for T where std::os::windows::io::AsRawHandle + std::os::windows::io::AsHandle {}
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        pub trait OsFile: std::os::fd::AsRawFd + std::os::fd::AsFd {}
        impl<T> OsFile for T where T: std::os::fd::AsRawFd + std::os::fd::AsFd {}
    } else {
        pub trait OsFile {}
        impl<T> OsFile for T {}
    }
}

cfg_if! {
    if #[cfg(windows)] {
        pub trait FromOsOwnedDescriptor: From<std::os::windows::io::OwnedHandle> {}
        impl<T> FromOsOwnedDescriptor for T where T: From<std::os::windows::io::OwnedHandle> {}
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        pub trait FromOsOnwedDescriptor: From<std::os::fd::OwnedFd> {}
        impl<T> FromOsOnwedDescriptor for T where T: From<std::os::fd::OwnedFd> {}
    } else {
        pub trait FromOsOwnedDescriptor {}
        impl<T> FromOsOwnedDescriptor for T {}
    }
}

pub trait RuntimeFile:
    OsFile + FromOsOnwedDescriptor + AsyncRead + AsyncWrite + AsyncSeek + Send + Sync
{
    type Runtime;

    fn open(
        runtime: &Self::Runtime,
        open_options: &OpenOptions,
        path: impl AsRef<Path>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;

    fn set_len(&self, size: u64) -> impl Future<Output = std::io::Result<()>> + Send;

    fn metadata(&self) -> impl Future<Output = std::io::Result<Metadata>> + Send;

    fn sync_all(&self) -> impl Future<Output = std::io::Result<()>> + Send;

    fn sync_data(&self) -> impl Future<Output = std::io::Result<()>> + Send;

    fn set_permissions(
        &self,
        permissions: Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}
