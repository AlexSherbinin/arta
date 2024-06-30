use cfg_if::cfg_if;
use futures::{AsyncRead, AsyncWrite};
use std::pin::Pin;
use tokio_util::compat::Compat;

pub struct TokioStdin<'a> {
    pub(super) inner: Compat<&'a mut tokio::process::ChildStdin>,
}

pub struct TokioStdout<'a> {
    pub(super) inner: Compat<&'a mut tokio::process::ChildStdout>,
}

pub struct TokioStderr<'a> {
    pub(super) inner: Compat<&'a mut tokio::process::ChildStderr>,
}

impl AsyncWrite for TokioStdin<'_> {
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

impl AsyncRead for TokioStdout<'_> {
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

impl AsyncRead for TokioStderr<'_> {
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

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawHandle for TokioStdin<'_> {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.get_ref().as_raw_handle();
            }
        }

        impl std::os::windows::io::AsRawHandle for TokioStdout<'_> {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.get_ref().as_raw_handle();
            }
        }

        impl std::os::windows::io::AsRawHandle for TokioStderr<'_> {
            fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
                self.inner.get_ref().as_raw_handle();
            }
        }

        impl std::os::windows::io::AsHandle for TokioStdin<'_> {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                self.inner.get_ref().as_handle()
            }
        }

        impl std::os::windows::io::AsHandle for TokioStdout<'_> {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                self.inner.get_ref().as_handle()
            }
        }

        impl std::os::windows::io::AsHandle for TokioStderr<'_> {
            fn as_handle(&self) -> std::os::windows::io::BorrowedHandle<'_> {
                self.inner.get_ref().as_handle()
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for TokioStdin<'_> {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.get_ref().as_raw_fd()
            }
        }

        impl std::os::fd::AsRawFd for TokioStdout<'_> {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.get_ref().as_raw_fd()
            }
        }

        impl std::os::fd::AsRawFd for TokioStderr<'_> {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.get_ref().as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for TokioStdin<'_> {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                self.inner.get_ref().as_fd()
            }
        }

        impl std::os::fd::AsFd for TokioStdout<'_> {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                self.inner.get_ref().as_fd()
            }
        }

        impl std::os::fd::AsFd for TokioStderr<'_> {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                self.inner.get_ref().as_fd()
            }
        }
    }
}
