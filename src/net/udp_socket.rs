use super::{NetRuntime, OsSocket, ToSocketAddrs};
use std::{
    future::Future,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
};

pub trait RuntimeUdpSocket: OsSocket {
    type Runtime: NetRuntime<UdpSocket = Self>;

    fn bind(
        runtime: &Self::Runtime,
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;

    fn connect(
        &self,
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    fn send(&self, buf: &[u8]) -> impl Future<Output = std::io::Result<usize>> + Send;

    fn send_to(
        &self,
        buf: &[u8],
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<usize>> + Send;

    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send;

    fn recv_from(
        &self,
        buf: &mut [u8],
    ) -> impl Future<Output = std::io::Result<(usize, SocketAddr)>> + Send;

    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    fn set_broadcast(&self, is_enabled: bool) -> std::io::Result<()>;

    fn broadcast(&self) -> std::io::Result<bool>;

    fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> std::io::Result<()>;

    fn leave_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> std::io::Result<()>;

    fn set_multicast_loop_v4(&self, is_enabled: bool) -> std::io::Result<()>;

    fn multicast_loop_v4(&self) -> std::io::Result<bool>;

    fn set_multicast_ttl_v4(&self, ttl: u32) -> std::io::Result<()>;

    fn multicast_ttl_v4(&self) -> std::io::Result<u32>;

    fn join_multicast_v6(&self, multiaddr: Ipv6Addr, interface: u32) -> std::io::Result<()>;

    fn leave_multicast_v6(&self, multiaddr: Ipv6Addr, interface: u32) -> std::io::Result<()>;

    fn set_multicast_loop_v6(&self, is_enabled: bool) -> std::io::Result<()>;

    fn multicast_loop_v6(&self) -> std::io::Result<bool>;

    fn ttl(&self) -> std::io::Result<u32>;

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;

    fn take_error(&self) -> std::io::Result<Option<std::io::Error>>;
}
