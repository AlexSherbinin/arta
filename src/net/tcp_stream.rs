use super::{NetRuntime, OsSocket, ToSocketAddrs};
use futures::{AsyncRead, AsyncWrite};
use std::{future::Future, net::SocketAddr, time::Duration};

/// Represents an async TCP stream between a local and a remote socket.
///
/// An async version of [`std::net::TcpStream`].
pub trait RuntimeTcpStream: OsSocket + AsyncRead + AsyncWrite + Send + Sync {
    /// An async runtime.
    type Runtime: NetRuntime<TcpStream = Self>;

    /// Opens a TCP connection to remote host.
    ///
    /// An async version of [`std::net::TcpStream::connect`].
    fn connect(
        runtime: &Self::Runtime,
        addr: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;

    /// Returns the socket address of the local half of this TCP connection.
    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    /// Returns the socket address of the remote peer of this TCP connection.
    fn peer_addr(&self) -> std::io::Result<SocketAddr>;

    /// Gets the value of the `SO_LINGER` option on this socket.
    ///
    /// For more information about this option, see [`RuntimeTcpStream::set_linger`].
    #[cfg(not(target_os = "wasi"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_os = "wasi"))))]
    fn linger(&self) -> std::io::Result<Option<Duration>>;

    /// Sets the value of the `SO_LINGER` option on this socket.
    ///
    /// This value controls how the socket is closed when data remains
    /// to be sent. If `SO_LINGER` is set, the socket will remain open
    /// for the specified duration as the system attempts to send pending data.
    /// Otherwise, the system may close the socket immediately, or wait for a
    /// default timeout.
    #[cfg(not(target_os = "wasi"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_os = "wasi"))))]
    fn set_linger(&self, linger: Option<Duration>) -> std::io::Result<()>;

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// For more information about this option, see [`RuntimeTcpStream::set_nodelay`].
    fn nodelay(&self) -> std::io::Result<bool>;

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// If set, this option disables the Nagle algorithm. This means that
    /// segments are always sent as soon as possible, even if there is only a
    /// small amount of data. When not set, data is buffered until there is a
    /// sufficient amount to send out, thereby avoiding the frequent sending of
    /// small packets.
    fn set_nodelay(&self, is_enabled: bool) -> std::io::Result<()>;

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`RuntimeTcpStream::set_ttl`].
    fn ttl(&self) -> std::io::Result<u32>;

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;

    /// Receives data on the socket from the remote address to which it is
    /// connected, without removing that data from the queue. On success,
    /// returns the number of bytes peeked.
    ///
    /// An async version of [`std::net::TcpStream::peek`].
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send;

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This will retrieve the stored error in the underlying socket, clearing
    /// the field in the process. This can be useful for checking errors between
    /// calls.
    fn take_error(&self) -> std::io::Result<Option<std::io::Error>>;
}
