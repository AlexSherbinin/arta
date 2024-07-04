use cfg_if::cfg_if;
use futures::{AsyncRead, AsyncWrite};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// Async-std specific async stdin implementation.
pub struct AsyncStdStdin<'a> {
    pub(super) inner: &'a mut async_std::process::ChildStdin,
}

/// Async-std specific async stdout implementation.
pub struct AsyncStdStdout<'a> {
    pub(super) inner: &'a mut async_std::process::ChildStdout,
}

/// Async-std specific async stderr implementation.
pub struct AsyncStdStderr<'a> {
    pub(super) inner: &'a mut async_std::process::ChildStderr,
}

impl AsyncWrite for AsyncStdStdin<'_> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Write::poll_write(Pin::new(self.inner), cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Write::poll_write_vectored(Pin::new(self.inner), cx, bufs)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        async_std::io::Write::poll_flush(Pin::new(self.inner), cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        async_std::io::Write::poll_close(Pin::new(self.inner), cx)
    }
}

impl AsyncRead for AsyncStdStdout<'_> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Read::poll_read(Pin::new(self.inner), cx, buf)
    }

    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Read::poll_read_vectored(Pin::new(self.inner), cx, bufs)
    }
}

impl AsyncRead for AsyncStdStderr<'_> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Read::poll_read(Pin::new(self.inner), cx, buf)
    }

    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Read::poll_read_vectored(Pin::new(self.inner), cx, bufs)
    }
}

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawHandle for AsyncStdStdin<'_> {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.as_raw_handle()
            }
        }

        impl std::os::windows::io::AsRawHandle for AsyncStdStdout<'_> {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.as_raw_handle()
            }
        }

        impl std::os::windows::io::AsRawHandle for AsyncStdStderr<'_> {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.as_raw_handle()
            }
        }

        impl std::os::windows::io::AsHandle for AsyncStdStdin<'_> {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                let raw_handle = std::os::windows::io::AsRawHandle::as_raw_handle(self);
                unsafe { std::os::windows::io::BorrowedHandle::borrow_raw(raw_handle) }
            }
        }

        impl std::os::windows::io::AsHandle for TokioStdout<'_> {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                let raw_handle = std::os::windows::io::AsRawHandle::as_raw_handle(self);
                unsafe { std::os::windows::io::BorrowedHandle::borrow_raw(raw_handle) }
            }
        }

        impl std::os::windows::io::AsHandle for TokioStderr<'_> {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                let raw_handle = std::os::windows::io::AsRawHandle::as_raw_handle(self);
                unsafe { std::os::windows::io::BorrowedHandle::borrow_raw(raw_handle) }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for AsyncStdStdin<'_> {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsRawFd for AsyncStdStdout<'_> {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsRawFd for AsyncStdStderr<'_> {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for AsyncStdStdin<'_> {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                let raw_fd = std::os::fd::AsRawFd::as_raw_fd(self);
                unsafe { std::os::fd::BorrowedFd::borrow_raw(raw_fd) }
            }
        }

        impl std::os::fd::AsFd for AsyncStdStdout<'_> {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                let raw_fd = std::os::fd::AsRawFd::as_raw_fd(self);
                unsafe { std::os::fd::BorrowedFd::borrow_raw(raw_fd) }
            }
        }

        impl std::os::fd::AsFd for AsyncStdStderr<'_> {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                let raw_fd = std::os::fd::AsRawFd::as_raw_fd(self);
                unsafe { std::os::fd::BorrowedFd::borrow_raw(raw_fd) }
            }
        }
    }
}
