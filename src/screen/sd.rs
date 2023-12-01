use anyhow::anyhow;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
#[derive(Debug)]
pub struct S2D<T> {
    inner: T,
    size_limit: usize,
}
impl<T> S2D<T> {
    /// Sets received datagram size limitation.
    pub fn set_size_limit(&mut self, new: usize) -> usize {
        std::mem::replace(&mut self.size_limit, new)
    }

    /// Creates a new [`S2D`] with provided stream.
    pub fn new(inner: T, size_limit: usize) -> Self {
        Self { inner, size_limit }
    }
}
impl<T: AsyncRead + Unpin> S2D<T> {
    /// Receives a datagram from the stream.
    pub async fn recv(&mut self) -> anyhow::Result<Vec<u8>> {
        let len = self.inner.read_u64_le().await? as usize;
        if len > self.size_limit {
            return Err(anyhow!("datagram is too big ({} bytes)", len));
        }
        let mut blob = vec![0u8; len];
        self.inner.read_exact(&mut blob).await?;

        Ok(blob)
    }
}
impl<T: AsyncWrite + Unpin> S2D<T> {
    /// Sends a datagram to the stream.
    pub async fn send(&mut self, blob: &[u8]) -> anyhow::Result<()> {
        self.inner.write_u64_le(blob.len() as _).await?;
        self.inner.write_all(blob).await?;

        Ok(())
    }
}
impl<T> AsRef<T> for S2D<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
impl<T> AsMut<T> for S2D<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
impl<T: AsyncRead + AsyncWrite + Unpin> From<T> for S2D<T> {
    fn from(inner: T) -> Self {
        Self::new(inner, 6 * 1024 * 1024)
    }
}
