use crate::AsyncStdGlobalRuntime;
use arta::fs::RuntimeFile;
use cfg_if::cfg_if;
use futures::{prelude::Future, AsyncRead, AsyncSeek, AsyncWrite, TryFutureExt};
use std::pin::Pin;

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawHandle for TokioFile {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.as_raw_handle()
            }
        }

        impl std::os::windows::io::AsHandle for TokioFile {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                let raw_handle = std::os::windows::io::AsRawHandle::as_raw_handle(self);
                unsafe { std::os::windows::io::BorrowedHandle::borrow_raw(raw_handle) }
            }
        }

        impl From<std::os::windows::io::OwnedHandle> for TokioFile {
            fn from(handle: std::os::windows::io::OwnedHandle) -> Self {
                Self { inner: tokio::fs::File::from_std(std::fs::File::from(handle)).compat() }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for AsyncStdFile {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for AsyncStdFile {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                let raw_fd = std::os::fd::AsRawFd::as_raw_fd(self);
                unsafe { std::os::fd::BorrowedFd::borrow_raw(raw_fd) }
            }
        }

        impl From<std::os::fd::OwnedFd> for AsyncStdFile {
            fn from(fd: std::os::fd::OwnedFd) -> Self {
                Self { inner: async_std::fs::File::from(std::fs::File::from(fd)) }
            }
        }
    }
}

/// Async-std specific [`RuntimeFile`] implementation.
pub struct AsyncStdFile {
    inner: async_std::fs::File,
}

impl AsyncWrite for AsyncStdFile {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_close(cx)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write_vectored(cx, bufs)
    }
}

impl AsyncRead for AsyncStdFile {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }

    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_read_vectored(cx, bufs)
    }
}

impl AsyncSeek for AsyncStdFile {
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        pos: std::io::SeekFrom,
    ) -> std::task::Poll<std::io::Result<u64>> {
        Pin::new(&mut self.inner).poll_seek(cx, pos)
    }
}

impl RuntimeFile for AsyncStdFile {
    type Runtime = AsyncStdGlobalRuntime;

    fn open(
        _runtime: &Self::Runtime,
        open_options: &std::fs::OpenOptions,
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized,
    {
        // async_std::fs::OpenOptions has no implementation to convert
        // from std primitves so we need to transmute it.
        let open_options: async_std::fs::OpenOptions =
            unsafe { std::mem::transmute(open_options.clone()) };
        let path: async_std::path::PathBuf = path.as_ref().to_owned().into();

        open_options.open(path).map_ok(|file| Self { inner: file })
    }

    fn set_len(&self, size: u64) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.set_len(size)
    }

    fn metadata(&self) -> impl Future<Output = std::io::Result<std::fs::Metadata>> + Send {
        self.inner.metadata()
    }

    fn sync_all(&self) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.sync_all()
    }

    fn sync_data(&self) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.sync_data()
    }

    fn set_permissions(
        &self,
        permissions: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        self.inner.set_permissions(permissions)
    }
}
