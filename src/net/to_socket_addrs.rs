use std::{
    future::Future,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

pub trait ToSocketAddrs<R>: Send + Sync
where
    R: Send + Sync,
{
    type Iterator: Iterator<Item = SocketAddr> + Send;
    type Future: Future<Output = std::io::Result<Self::Iterator>> + Send;

    fn to_socket_addrs(&self, runtime: &R) -> Self::Future;

    fn for_each_resolved_addr_until_success<T, Fut>(
        self,
        runtime: &R,
        f: impl Fn(SocketAddr) -> Fut + Send + Sync,
    ) -> impl Future<Output = std::io::Result<T>> + Send
    where
        Fut: Future<Output = std::io::Result<T>> + Send,
        Self: Sized,
    {
        async move {
            let mut last_err = None;

            for addr in self.to_socket_addrs(runtime).await? {
                match f(addr).await {
                    Ok(result) => return Ok(result),
                    Err(err) => last_err = Some(err),
                }
            }

            Err(last_err.unwrap_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "no address was resolved")
            }))
        }
    }
}

impl<R> ToSocketAddrs<R> for SocketAddr
where
    R: Send + Sync,
{
    type Iterator = std::iter::Once<SocketAddr>;
    type Future = std::future::Ready<std::io::Result<Self::Iterator>>;

    fn to_socket_addrs(&self, _runtime: &R) -> Self::Future {
        std::future::ready(Ok(std::iter::once(*self)))
    }
}

impl<R> ToSocketAddrs<R> for SocketAddrV4
where
    R: Send + Sync,
{
    type Iterator = std::iter::Once<SocketAddr>;
    type Future = std::future::Ready<std::io::Result<Self::Iterator>>;

    fn to_socket_addrs(&self, runtime: &R) -> Self::Future {
        SocketAddr::V4(*self).to_socket_addrs(runtime)
    }
}

impl<R> ToSocketAddrs<R> for SocketAddrV6
where
    R: Send + Sync,
{
    type Iterator = std::iter::Once<SocketAddr>;
    type Future = std::future::Ready<std::io::Result<Self::Iterator>>;

    fn to_socket_addrs(&self, runtime: &R) -> Self::Future {
        SocketAddr::V6(*self).to_socket_addrs(runtime)
    }
}

impl<R> ToSocketAddrs<R> for (IpAddr, u16)
where
    R: Send + Sync,
{
    type Iterator = std::iter::Once<SocketAddr>;
    type Future = std::future::Ready<std::io::Result<Self::Iterator>>;

    fn to_socket_addrs(&self, runtime: &R) -> Self::Future {
        SocketAddr::new(self.0, self.1).to_socket_addrs(runtime)
    }
}

impl<R> ToSocketAddrs<R> for (Ipv4Addr, u16)
where
    R: Send + Sync,
{
    type Iterator = std::iter::Once<SocketAddr>;
    type Future = std::future::Ready<std::io::Result<Self::Iterator>>;

    fn to_socket_addrs(&self, runtime: &R) -> Self::Future {
        (IpAddr::V4(self.0), self.1).to_socket_addrs(runtime)
    }
}

impl<R> ToSocketAddrs<R> for (Ipv6Addr, u16)
where
    R: Send + Sync,
{
    type Iterator = std::iter::Once<SocketAddr>;
    type Future = std::future::Ready<std::io::Result<Self::Iterator>>;

    fn to_socket_addrs(&self, runtime: &R) -> Self::Future {
        (IpAddr::V6(self.0), self.1).to_socket_addrs(runtime)
    }
}

impl<'a, R> ToSocketAddrs<R> for &'a [SocketAddr]
where
    R: Send + Sync,
{
    type Iterator = std::iter::Copied<std::slice::Iter<'a, SocketAddr>>;
    type Future = std::future::Ready<std::io::Result<Self::Iterator>>;

    fn to_socket_addrs(&self, _runtime: &R) -> Self::Future {
        std::future::ready(Ok(self.iter().copied()))
    }
}
