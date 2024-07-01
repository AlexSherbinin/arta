//! Networking primitives for TCP/UDP communication.

mod tcp_listener;
mod tcp_stream;
mod to_socket_addrs;
mod udp_socket;

pub use tcp_listener::*;
pub use tcp_stream::*;
pub use to_socket_addrs::*;
pub use udp_socket::*;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        /// Represents a socket that implements OS specific methods.
        pub trait OsSocket: std::os::windows::io::AsRawSocket + std::os::windows::io::AsSocket + From<std::os::windows::io::OwnedSocket> {}
        impl<T> OsSocket for T where T: std::os::windows::io::AsRawSocket + std::os::windows::io::AsSocket {}
    } else if #[cfg(any(unix, target_os = "wasi"))]{
        /// Represents a socket that implements OS specific methods.
        pub trait OsSocket: std::os::fd::AsRawFd + std::os::fd::AsFd + From<std::os::fd::OwnedFd> {}
        impl<T> OsSocket for T where T: std::os::fd::AsRawFd + std::os::fd::AsFd + From<std::os::fd::OwnedFd> {}
    } else {
        /// Represents a socket that implements OS specific methods.
        pub trait OsSocket {}
        impl<T> OsSocket for T {}
    }
}

/// Represents an async runtime that supports asynchronous networking.
pub trait NetRuntime: Send + Sync {
    /// Runtime's tcp listener.
    type TcpListener: RuntimeTcpListener<Runtime = Self>;
    /// Runtime's tcp stream.
    type TcpStream: RuntimeTcpStream<Runtime = Self>;
    /// Runtime's udp socket.
    type UdpSocket: RuntimeUdpSocket<Runtime = Self>;
}
