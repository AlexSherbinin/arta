use crate::runtimes::TokioGlobalRuntime;
use arta::fs::RuntimeFile;
use cfg_if::cfg_if;
use futures::{AsyncRead, AsyncSeek, AsyncWrite};
use std::{
    fs::{Metadata, OpenOptions},
    future::Future,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawHandle for TokioFile {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.get_ref().as_raw_handle()
            }
        }

        impl std::os::windows::io::AsHandle for TokioFile {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                self.inner.get_ref().as_handle()
            }
        }

        impl From<std::os::windows::io::OwnedHandle> for TokioFile {
            fn from(handle: std::os::windows::io::OwnedHandle) -> Self {
                Self { inner: tokio::fs::File::from_std(std::fs::File::from(handle)).compat() }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for TokioFile {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.get_ref().as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for TokioFile {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                self.inner.get_ref().as_fd()
            }
        }

        impl From<std::os::fd::OwnedFd> for TokioFile {
            fn from(fd: std::os::fd::OwnedFd) -> Self {
                Self { inner: tokio::fs::File::from_std(std::fs::File::from(fd)).compat() }
            }
        }
    }
}

pub struct TokioFile {
    inner: Compat<tokio::fs::File>,
}

impl AsyncRead for TokioFile {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for TokioFile {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_close(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_write_vectored(cx, bufs)
    }
}

impl AsyncSeek for TokioFile {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: std::io::SeekFrom,
    ) -> Poll<std::io::Result<u64>> {
        Pin::new(&mut self.get_mut().inner).poll_seek(cx, pos)
    }
}

impl RuntimeFile for TokioFile {
    type Runtime = TokioGlobalRuntime;

    fn open(
        _runtime: &Self::Runtime,
        open_options: &OpenOptions,
        path: impl AsRef<Path>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized,
    {
        let path = path.as_ref().to_owned();
        async {
            let open_options = tokio::fs::OpenOptions::from(open_options.clone());
            open_options.open(path).await.map(|inner| Self {
                inner: inner.compat(),
            })
        }
    }

    fn set_len(&self, size: u64) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.get_ref().set_len(size)
    }

    fn metadata(&self) -> impl Future<Output = std::io::Result<Metadata>> + Send {
        self.inner.get_ref().metadata()
    }

    fn sync_all(&self) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.get_ref().sync_all()
    }

    fn sync_data(&self) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.get_ref().sync_data()
    }

    fn set_permissions(
        &self,
        permissions: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.get_ref().set_permissions(permissions)
    }
}
