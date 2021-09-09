//! Commonly used listeners.

mod combined;
mod tcp;
#[cfg(feature = "tls")]
mod tls;

use std::fmt::Display;

pub use combined::{CombinedAcceptor, CombinedListener, CombinedStream};
pub use tcp::{TcpAcceptor, TcpListener};
#[cfg(feature = "tls")]
pub use tls::{TlsAcceptor, TlsConfig, TlsListener};
use tokio::io::{AsyncRead, AsyncWrite, Result as IoResult};

/// Represents a acceptor type.
#[async_trait::async_trait]
pub trait Acceptor: Send + Sync {
    /// Address type.
    type Addr: Send + Display + 'static;

    /// IO stream type.
    type Io: AsyncRead + AsyncWrite + Send + Unpin + 'static;

    /// Returns the local address that this listener is bound to.
    fn local_addr(&self) -> IoResult<Self::Addr>;

    /// Accepts a new incoming connection from this listener.
    ///
    /// This function will yield once a new TCP connection is established. When
    /// established, the corresponding IO stream and the remote peer’s
    /// address will be returned.
    async fn accept(&mut self) -> IoResult<(Self::Io, Self::Addr)>;
}

/// Represents a type that can be converted to a acceptor.
#[async_trait::async_trait]
pub trait IntoAcceptor: Send {
    /// The acceptor type.
    type Acceptor: Acceptor;

    /// Create a acceptor instance.
    async fn into_acceptor(self) -> IoResult<Self::Acceptor>;

    /// Combine two listeners.
    ///
    /// You can call this function multiple times to combine more listeners.
    fn combine<T>(self, other: T) -> CombinedListener<Self, T>
    where
        Self: Sized,
    {
        CombinedListener::new(self, other)
    }

    /// Consume this listener and return a new TLS listener.
    #[cfg(feature = "tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
    fn tls(self, config: TlsConfig) -> TlsListener<Self>
    where
        Self: Sized,
    {
        TlsListener::new(config, self)
    }
}
