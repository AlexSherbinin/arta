use super::TokioTcpStream;
use crate::TokioGlobalRuntime;
use arta::net::{NetRuntime, RuntimeTcpListener};
use cfg_if::cfg_if;
use futures::TryFutureExt;
use std::{future::Future, net::SocketAddr};
use tokio_util::compat::TokioAsyncReadCompatExt;

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawSocket for TokioTcpListener {
            fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
                self.inner.as_raw_socket()
            }
        }

        impl std::os::windows::io::AsSocket for TokioTcpListener {
            fn as_socket(&self) -> std::os::windows::io::BorrowedSocket<'_> {
                self.inner.as_socket()
            }
        }

        impl From<std::os::windows::io::OwnedSocket> for TokioTcpListener {
            fn from(socket: std::os::windows::io::OwnedSocket) -> Self {
                Self { inner: tokio::net::TcpListener::from_std(std::net::TcpListener::from(socket)).unwrap() }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for TokioTcpListener {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for TokioTcpListener {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                self.inner.as_fd()
            }
        }

        impl From<std::os::fd::OwnedFd> for TokioTcpListener {
            fn from(fd: std::os::fd::OwnedFd) -> Self {
                Self { inner: tokio::net::TcpListener::from_std(std::net::TcpListener::from(fd)).unwrap() }
            }
        }
    }
}

/// Tokio specific [`RuntimeTcpListener`] implementation.
pub struct TokioTcpListener {
    inner: tokio::net::TcpListener,
}

impl RuntimeTcpListener for TokioTcpListener {
    type Runtime = TokioGlobalRuntime;

    fn accept(
        &self,
    ) -> impl Future<Output = std::io::Result<(<Self::Runtime as NetRuntime>::TcpStream, SocketAddr)>>
           + Send {
        self.inner.accept().map_ok(|(stream, addr)| {
            (
                TokioTcpStream {
                    inner: stream.compat(),
                },
                addr,
            )
        })
    }

    fn bind(
        runtime: &Self::Runtime,
        addr: impl arta::net::ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized,
    {
        addr.for_each_resolved_addr_until_success(runtime, |addr| {
            tokio::net::TcpListener::bind(addr).map_ok(|listener| Self { inner: listener })
        })
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    fn ttl(&self) -> std::io::Result<u32> {
        self.inner.ttl()
    }

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.inner.set_ttl(ttl)
    }
}
