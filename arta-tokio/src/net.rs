mod tcp_listener;
mod tcp_stream;
mod udp_socket;

pub use tcp_listener::*;
pub use tcp_stream::*;
pub use udp_socket::*;

use crate::runtimes::TokioGlobalRuntime;
use arta::net::NetRuntime;

impl NetRuntime for TokioGlobalRuntime {
    type TcpListener = TokioTcpListener;
    type TcpStream = TokioTcpStream;
    type UdpSocket = TokioUdpSocket;
}
