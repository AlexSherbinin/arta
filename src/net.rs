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
        pub trait OsSocket: std::os::windows::io::AsRawSocket + std::os::windows::io::AsSocket + From<std::os::windows::io::OwnedSocket> {}
        impl<T> OsSocket for T where T: std::os::windows::io::AsRawSocket + std::os::windows::io::AsSocket {}
    } else if #[cfg(any(unix, target_os = "wasi"))]{
        pub trait OsSocket: std::os::fd::AsRawFd + std::os::fd::AsFd + From<std::os::fd::OwnedFd> {}
        impl<T> OsSocket for T where T: std::os::fd::AsRawFd + std::os::fd::AsFd + From<std::os::fd::OwnedFd> {}
    } else {
        pub trait OsSocket {}
        impl<T> OsSocket for T {}
    }
}

pub trait NetRuntime: Send + Sync {
    type TcpListener: RuntimeTcpListener<Runtime = Self>;
    type TcpStream: RuntimeTcpStream<Runtime = Self>;
    type UdpSocket: RuntimeUdpSocket<Runtime = Self>;
}
