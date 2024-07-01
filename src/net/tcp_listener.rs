use super::{NetRuntime, OsSocket, ToSocketAddrs};
use std::{future::Future, net::SocketAddr};

/// Represents an async TCP socket server, listening for connections.
///
/// An async version of [`std::net::TcpListener`].
pub trait RuntimeTcpListener: OsSocket + Send + Sync {
    /// An async runtime.
    type Runtime: NetRuntime<TcpListener = Self>;

    /// Accept a new incoming connection from this listener.
    ///
    /// An async version of [`std::net::TcpListener::accept`].
    fn accept(
        &self,
    ) -> impl Future<Output = std::io::Result<(<Self::Runtime as NetRuntime>::TcpStream, SocketAddr)>>
           + Send;

    /// Creates a new `TcpListener` which will be bound to the specified address.
    ///
    /// An async version of [`std::net::TcpListener::bind`].
    fn bind(
        runtime: &Self::Runtime,
        addr: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;

    /// Returns the local socket address of this listener.
    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`RuntimeTcpListener::set_ttl`].
    fn ttl(&self) -> std::io::Result<u32>;

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;
}
