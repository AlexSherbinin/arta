use super::AsyncStdTcpStream;
use crate::AsyncStdGlobalRuntime;
use arta::net::{NetRuntime, RuntimeTcpListener, ToSocketAddrs};
use cfg_if::cfg_if;
use futures::{prelude::Future, TryFutureExt};
use socket2::SockRef;
use std::net::SocketAddr;

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawSocket for AsyncStdTcpListener {
            fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
                self.inner.as_raw_socket()
            }
        }

        impl std::os::windows::io::AsSocket for AsyncStdTcpListener {
            fn as_socket(&self) -> std::os::windows::io::BorrowedSocket<'_> {
                let raw_socket = std::os::windows::io::AsRawSocket::as_raw_socket(self);
                unsafe { std::os::windows::io::BorrowedSocket::borrow_raw(raw_socket) }
            }
        }

        impl From<std::os::windows::io::OwnedSocket> for AsyncStdTcpListener {
            fn from(socket: std::os::windows::io::OwnedSocket) -> Self {
                Self {
                    inner: async_std::net::TcpListener::from(std::net::TcpListener::from(socket))
                }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for AsyncStdTcpListener {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for AsyncStdTcpListener {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                let raw_fd = std::os::fd::AsRawFd::as_raw_fd(self);
                unsafe { std::os::fd::BorrowedFd::borrow_raw(raw_fd) }
            }
        }

        impl From<std::os::fd::OwnedFd> for AsyncStdTcpListener {
            fn from(fd: std::os::fd::OwnedFd) -> Self {
                Self {
                    inner: async_std::net::TcpListener::from(std::net::TcpListener::from(fd))
                }
            }
        }
    }
}

/// Async-std specific [`RuntimeTcpListener`] implementation.
pub struct AsyncStdTcpListener {
    inner: async_std::net::TcpListener,
}

impl RuntimeTcpListener for AsyncStdTcpListener {
    type Runtime = AsyncStdGlobalRuntime;

    fn accept(
        &self,
    ) -> impl Future<Output = std::io::Result<(<Self::Runtime as NetRuntime>::TcpStream, SocketAddr)>>
           + Send {
        self.inner
            .accept()
            .map_ok(|(stream, addr)| (AsyncStdTcpStream { inner: stream }, addr))
    }

    fn bind(
        runtime: &Self::Runtime,
        addr: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized,
    {
        addr.for_each_resolved_addr_until_success(runtime, |addr| {
            async_std::net::TcpListener::bind(addr).map_ok(|listener| Self { inner: listener })
        })
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    fn ttl(&self) -> std::io::Result<u32> {
        SockRef::from(self).ttl()
    }

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        SockRef::from(self).set_ttl(ttl)
    }
}
