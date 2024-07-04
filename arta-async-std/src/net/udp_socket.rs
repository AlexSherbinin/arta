use crate::AsyncStdGlobalRuntime;
use arta::net::{RuntimeUdpSocket, ToSocketAddrs};
use cfg_if::cfg_if;
use futures::{prelude::Future, TryFutureExt};
use socket2::SockRef;
use std::net::SocketAddr;

cfg_if! {
    if #[cfg(windows)] {
        impl std::os::windows::io::AsRawSocket for AsyncStdUdpSocket {
            fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
                self.inner.as_raw_socket()
            }
        }

        impl std::os::windows::io::AsSocket for AsyncStdUdpSocket {
            fn as_socket(&self) -> std::os::windows::io::BorrowedSocket<'_> {
                let raw_socket = std::os::windows::io::AsRawSocket::as_raw_socket(self);
                unsafe { std::os::windows::io::BorrowedSocket::borrow_raw(raw_socket) }
            }
        }

        impl From<std::os::windows::io::OwnedSocket> for AsyncStdUdpSocket {
            fn from(socket: std::os::windows::io::OwnedSocket) -> Self {
                Self {
                    inner: async_std::net::UdpSocket::from(std::net::UdpSocket::from(socket))
                }
            }
        }
    } else if #[cfg(any(unix, target_os = "wasi"))] {
        impl std::os::fd::AsRawFd for AsyncStdUdpSocket {
            fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
                self.inner.as_raw_fd()
            }
        }

        impl std::os::fd::AsFd for AsyncStdUdpSocket {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                let raw_fd = std::os::fd::AsRawFd::as_raw_fd(self);
                unsafe { std::os::fd::BorrowedFd::borrow_raw(raw_fd) }
            }
        }

        impl From<std::os::fd::OwnedFd> for AsyncStdUdpSocket {
            fn from(fd: std::os::fd::OwnedFd) -> Self {
                Self {
                    inner: async_std::net::UdpSocket::from(std::net::UdpSocket::from(fd))
                }
            }
        }
    }
}

/// Async-std specific [`RuntimeUdpSocket`] implementation.
pub struct AsyncStdUdpSocket {
    inner: async_std::net::UdpSocket,
}

impl RuntimeUdpSocket for AsyncStdUdpSocket {
    type Runtime = AsyncStdGlobalRuntime;

    fn bind(
        runtime: &Self::Runtime,
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized,
    {
        addrs.for_each_resolved_addr_until_success(runtime, |addr| {
            async_std::net::UdpSocket::bind(addr).map_ok(|socket| Self { inner: socket })
        })
    }

    fn connect(
        &self,
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<()>> + Send {
        addrs.for_each_resolved_addr_until_success(&AsyncStdGlobalRuntime, |addr| {
            self.inner.connect(addr)
        })
    }

    fn send(&self, buf: &[u8]) -> impl Future<Output = std::io::Result<usize>> + Send {
        self.inner.send(buf)
    }

    async fn send_to(
        &self,
        buf: &[u8],
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> std::io::Result<usize> {
        if let Some(addr) = addrs.to_socket_addrs(&AsyncStdGlobalRuntime).await?.next() {
            self.inner.send_to(buf, addr).await
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "no address was resolved",
            ))
        }
    }

    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send {
        self.inner.recv(buf)
    }

    fn recv_from(
        &self,
        buf: &mut [u8],
    ) -> impl Future<Output = std::io::Result<(usize, SocketAddr)>> + Send {
        self.inner.recv_from(buf)
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    fn set_broadcast(&self, is_enabled: bool) -> std::io::Result<()> {
        self.inner.set_broadcast(is_enabled)
    }

    fn broadcast(&self) -> std::io::Result<bool> {
        self.inner.broadcast()
    }

    fn join_multicast_v4(
        &self,
        multiaddr: std::net::Ipv4Addr,
        interface: std::net::Ipv4Addr,
    ) -> std::io::Result<()> {
        self.inner.join_multicast_v4(multiaddr, interface)
    }

    fn leave_multicast_v4(
        &self,
        multiaddr: std::net::Ipv4Addr,
        interface: std::net::Ipv4Addr,
    ) -> std::io::Result<()> {
        self.inner.leave_multicast_v4(multiaddr, interface)
    }

    fn set_multicast_loop_v4(&self, is_enabled: bool) -> std::io::Result<()> {
        self.inner.set_multicast_loop_v4(is_enabled)
    }

    fn multicast_loop_v4(&self) -> std::io::Result<bool> {
        self.inner.multicast_loop_v4()
    }

    fn set_multicast_ttl_v4(&self, ttl: u32) -> std::io::Result<()> {
        self.inner.set_multicast_ttl_v4(ttl)
    }

    fn multicast_ttl_v4(&self) -> std::io::Result<u32> {
        todo!()
    }

    fn join_multicast_v6(
        &self,
        multiaddr: std::net::Ipv6Addr,
        interface: u32,
    ) -> std::io::Result<()> {
        self.inner.join_multicast_v6(&multiaddr, interface)
    }

    fn leave_multicast_v6(
        &self,
        multiaddr: std::net::Ipv6Addr,
        interface: u32,
    ) -> std::io::Result<()> {
        self.inner.leave_multicast_v6(&multiaddr, interface)
    }

    fn set_multicast_loop_v6(&self, is_enabled: bool) -> std::io::Result<()> {
        self.inner.set_multicast_loop_v6(is_enabled)
    }

    fn multicast_loop_v6(&self) -> std::io::Result<bool> {
        self.inner.multicast_loop_v6()
    }

    fn ttl(&self) -> std::io::Result<u32> {
        self.inner.ttl()
    }

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.inner.set_ttl(ttl)
    }

    fn take_error(&self) -> std::io::Result<Option<std::io::Error>> {
        SockRef::from(self).take_error()
    }
}
