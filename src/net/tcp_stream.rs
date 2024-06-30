use super::{NetRuntime, OsSocket, ToSocketAddrs};
use futures::{AsyncRead, AsyncWrite};
use std::{future::Future, net::SocketAddr, time::Duration};

pub trait RuntimeTcpStream: OsSocket + AsyncRead + AsyncWrite + Send + Sync {
    type Runtime: NetRuntime<TcpStream = Self>;

    fn connect(
        runtime: &Self::Runtime,
        addr: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;
    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    fn peer_addr(&self) -> std::io::Result<SocketAddr>;

    fn linger(&self) -> std::io::Result<Option<Duration>>;

    fn set_linger(&self, linger: Option<Duration>) -> std::io::Result<()>;

    fn nodelay(&self) -> std::io::Result<bool>;

    fn set_nodelay(&self, is_enabled: bool) -> std::io::Result<()>;

    fn ttl(&self) -> std::io::Result<u32>;

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;

    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send;

    fn take_error(&self) -> std::io::Result<Option<std::io::Error>>;
}
