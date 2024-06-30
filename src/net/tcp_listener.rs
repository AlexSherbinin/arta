use super::{NetRuntime, OsSocket, ToSocketAddrs};
use std::{future::Future, net::SocketAddr};

pub trait RuntimeTcpListener: OsSocket + Send + Sync {
    type Runtime: NetRuntime<TcpListener = Self>;

    fn accept(
        &self,
    ) -> impl Future<Output = std::io::Result<(<Self::Runtime as NetRuntime>::TcpStream, SocketAddr)>>
           + Send;

    fn bind(
        runtime: &Self::Runtime,
        addr: impl ToSocketAddrs<Self::Runtime>,
    ) -> impl Future<Output = std::io::Result<Self>> + Send
    where
        Self: Sized;

    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    fn ttl(&self) -> std::io::Result<u32>;

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;
}
