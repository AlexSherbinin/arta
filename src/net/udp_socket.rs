use super::{NetRuntime, OsSocket, ToSocketAddrs};
use std::{
    future::Future,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
};

/// An async UDP socket.
///
/// An async version of [`std::net::UdpSocket`].
pub trait RuntimeUdpSocket: OsSocket {
    /// An async runtime.
    type Runtime: NetRuntime<UdpSocket = Self>;

    /// Creates a UDP socket from the given address.
    ///
    /// An async version of [`std::net::UdpSocket::bind`].
    fn bind(
        runtime: &Self::Runtime,
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;

    /// Connects this UDP socket to a remote address, allowing the `send` and
    /// `recv` syscalls to be used to send data and also applies filters to only
    /// receive data from the specified address.
    ///
    /// An async version of [`std::net::UdpSocket::connect`].
    fn connect(
        &self,
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<()>> + Send;

    /// Sends data on the socket to the remote address to which it is connected.
    /// On success, returns the number of bytes written.
    ///
    /// An async version of [`std::net::UdpSocket::send`].
    fn send(&self, buf: &[u8]) -> impl Future<Output = std::io::Result<usize>> + Send;

    /// Sends data on the socket to the given address. On success, returns the
    /// number of bytes written.
    ///
    /// An async version of [`std::net::UdpSocket::send_to`].
    fn send_to(
        &self,
        buf: &[u8],
        addrs: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<usize>> + Send;

    /// Receives a single datagram message on the socket from the remote address to
    /// which it is connected. On success, returns the number of bytes read.
    ///
    /// An async version of [`std::net::UdpSocket::recv`].
    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send;

    /// Receives a single datagram message on the socket. On success, returns the number
    /// of bytes read and the origin.
    ///
    /// An async version of [`std::net::UdpSocket::recv_from`].
    fn recv_from(
        &self,
        buf: &mut [u8],
    ) -> impl Future<Output = std::io::Result<(usize, SocketAddr)>> + Send;

    /// Returns the socket address that this socket was created from.
    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    fn set_broadcast(&self, is_enabled: bool) -> std::io::Result<()>;

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// For more information about this option, see [`RuntimeUdpSocket::set_broadcast`].
    fn broadcast(&self) -> std::io::Result<bool>;

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> std::io::Result<()>;

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    fn leave_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> std::io::Result<()>;

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v4(&self, is_enabled: bool) -> std::io::Result<()>;

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v4(&self) -> std::io::Result<bool>;

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn set_multicast_ttl_v4(&self, ttl: u32) -> std::io::Result<()>;

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn multicast_ttl_v4(&self) -> std::io::Result<u32>;

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    fn join_multicast_v6(&self, multiaddr: Ipv6Addr, interface: u32) -> std::io::Result<()>;

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    fn leave_multicast_v6(&self, multiaddr: Ipv6Addr, interface: u32) -> std::io::Result<()>;

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v6(&self, is_enabled: bool) -> std::io::Result<()>;

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v6(&self) -> std::io::Result<bool>;

    /// Gets the value of the `IP_TTL` option for this socket.
    fn ttl(&self) -> std::io::Result<u32>;

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;

    /// Gets the value of the `SO_ERROR` option on this socket.
    fn take_error(&self) -> std::io::Result<Option<std::io::Error>>;
}
