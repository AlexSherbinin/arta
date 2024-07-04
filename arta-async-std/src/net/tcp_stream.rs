use crate::AsyncStdGlobalRuntime;
use arta::net::RuntimeTcpStream;
use cfg_if::cfg_if;
use futures::{prelude::Future, AsyncRead, AsyncWrite, TryFutureExt};
use socket2::SockRef;
use std::{
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawSocket for AsyncStdTcpStream {
            fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
                self.inner.as_raw_socket()
            }
        }

        impl std::os::windows::io::AsSocket for AsyncStdTcpStream {
            fn as_socket(&self) -> std::os::windows::io::BorrowedSocket<'_> {
                let raw_socket = std::os::windows::io::AsRawSocket::as_raw_socket(self);
                unsafe { std::os::windows::io::BorrowedSocket::borrow_raw(raw_socket) }
            }
        }

        impl From<std::os::windows::io::OwnedSocket> for AsyncStdTcpStream {
            fn from(socket: std::os::windows::io::OwnedSocket) -> Self {
                Self {
                    inner: async_std::net::TcpStream::from(std::net::TcpStream::from(socket))
                }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for AsyncStdTcpStream {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for AsyncStdTcpStream {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                let raw_fd = std::os::fd::AsRawFd::as_raw_fd(self);
                unsafe { std::os::fd::BorrowedFd::borrow_raw(raw_fd) }
            }
        }

        impl From<std::os::fd::OwnedFd> for AsyncStdTcpStream {
            fn from(fd: std::os::fd::OwnedFd) -> Self {
                Self {
                    inner: async_std::net::TcpStream::from(std::net::TcpStream::from(fd))
                }
            }
        }
    }
}

/// Async-std specific [`RuntimeTcpStream`] implementation.
pub struct AsyncStdTcpStream {
    pub(super) inner: async_std::net::TcpStream,
}

impl RuntimeTcpStream for AsyncStdTcpStream {
    type Runtime = AsyncStdGlobalRuntime;

    fn connect(
        runtime: &Self::Runtime,
        addr: impl arta::net::ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized,
    {
        addr.for_each_resolved_addr_until_success(runtime, |addr| {
            async_std::net::TcpStream::connect(addr).map_ok(|stream| Self { inner: stream })
        })
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.peer_addr()
    }

    #[cfg(not(target_os = "wasi"))]
    fn linger(&self) -> std::io::Result<Option<Duration>> {
        SockRef::from(self).linger()
    }

    #[cfg(not(target_os = "wasi"))]
    fn set_linger(&self, linger: Option<Duration>) -> std::io::Result<()> {
        SockRef::from(self).set_linger(linger)
    }

    fn nodelay(&self) -> std::io::Result<bool> {
        self.inner.nodelay()
    }

    fn set_nodelay(&self, is_enabled: bool) -> std::io::Result<()> {
        self.inner.set_nodelay(is_enabled)
    }

    fn ttl(&self) -> std::io::Result<u32> {
        self.inner.ttl()
    }

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.inner.set_ttl(ttl)
    }

    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send {
        self.inner.peek(buf)
    }

    fn take_error(&self) -> std::io::Result<Option<std::io::Error>> {
        SockRef::from(self).take_error()
    }
}

impl AsyncRead for AsyncStdTcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Read::poll_read(Pin::new(&mut self.inner), cx, buf)
    }

    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [std::io::IoSliceMut<'_>],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Read::poll_read_vectored(Pin::new(&mut self.inner), cx, bufs)
    }
}

impl AsyncWrite for AsyncStdTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Write::poll_write(Pin::new(&mut self.inner), cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        async_std::io::Write::poll_write_vectored(Pin::new(&mut self.inner), cx, bufs)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        async_std::io::Write::poll_flush(Pin::new(&mut self.inner), cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        async_std::io::Write::poll_close(Pin::new(&mut self.inner), cx)
    }
}
