use crate::TokioGlobalRuntime;
use arta::net::{RuntimeTcpStream, ToSocketAddrs};
use cfg_if::cfg_if;
use futures::{prelude::Future, AsyncRead, AsyncWrite, TryFutureExt};
use std::{
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawSocket for TokioTcpStream {
            fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
                self.inner.get_ref().as_raw_socket()
            }
        }

        impl std::os::windows::io::AsSocket for TokioTcpStream {
            fn as_socket(&self) -> std::os::windows::io::BorrowedSocket<'_> {
                self.inner.get_ref().as_socket()
            }
        }

        impl From<std::os::windows::io::OwnedSocket> for TokioTcpStream {
            fn from(socket: std::os::windows::io::OwnedSocket) -> Self {
                Self { inner: tokio::net::TcpStream::from_std(std::net::TcpStream::from(socket)).unwrap().compat() }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for TokioTcpStream {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.get_ref().as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for TokioTcpStream {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                self.inner.get_ref().as_fd()
            }
        }

        impl From<std::os::fd::OwnedFd> for TokioTcpStream {
            fn from(fd: std::os::fd::OwnedFd) -> Self {
                Self { inner: tokio::net::TcpStream::from_std(std::net::TcpStream::from(fd)).unwrap().compat() }
            }
        }
    }
}

/// Tokio specific [`RuntimeTcpStream`] implementation.
pub struct TokioTcpStream {
    pub(super) inner: Compat<tokio::net::TcpStream>,
}

impl AsyncRead for TokioTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for TokioTcpStream {
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

impl RuntimeTcpStream for TokioTcpStream {
    type Runtime = TokioGlobalRuntime;

    fn connect(
        runtime: &Self::Runtime,
        addr: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized,
    {
        addr.for_each_resolved_addr_until_success(runtime, |addr| {
            tokio::net::TcpStream::connect(addr).map_ok(|stream| Self {
                inner: stream.compat(),
            })
        })
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.get_ref().local_addr()
    }

    fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.get_ref().peer_addr()
    }

    fn linger(&self) -> std::io::Result<Option<std::time::Duration>> {
        self.inner.get_ref().linger()
    }

    fn set_linger(&self, linger: Option<std::time::Duration>) -> std::io::Result<()> {
        self.inner.get_ref().set_linger(linger)
    }

    fn nodelay(&self) -> std::io::Result<bool> {
        self.inner.get_ref().nodelay()
    }

    fn set_nodelay(&self, is_enabled: bool) -> std::io::Result<()> {
        self.inner.get_ref().set_nodelay(is_enabled)
    }

    fn ttl(&self) -> std::io::Result<u32> {
        self.inner.get_ref().ttl()
    }

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.inner.get_ref().set_ttl(ttl)
    }

    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> {
        self.inner.get_ref().peek(buf)
    }

    fn take_error(&self) -> std::io::Result<Option<std::io::Error>> {
        self.inner.get_ref().take_error()
    }
}
