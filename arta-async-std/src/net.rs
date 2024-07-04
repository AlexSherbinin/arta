//! Async-std specific TCP/UDP communication implementation.
mod tcp_listener;
mod tcp_stream;
mod udp_socket;

use arta::net::NetRuntime;
pub use tcp_listener::*;
pub use tcp_stream::*;
pub use udp_socket::*;

use crate::AsyncStdGlobalRuntime;

impl NetRuntime for AsyncStdGlobalRuntime {
    type TcpListener = AsyncStdTcpListener;
    type TcpStream = AsyncStdTcpStream;
    type UdpSocket = AsyncStdUdpSocket;
}
