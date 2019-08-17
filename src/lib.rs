use async_std::net::TcpStream;
use async_std::os::unix::net::UnixStream;
use async_std::task::{Context, Poll};
use futures::io::{AsyncRead, AsyncWrite};
use std::io;
use std::pin::Pin;

pub struct TokioCompat<T>(T);

pub trait TokioCompatExt: AsyncRead + AsyncWrite + Sized {
    #[inline]
    fn compat(self) -> TokioCompat<Self> {
        TokioCompat(self)
    }
}

impl<T: AsyncRead + Unpin> tokio::io::AsyncRead for TokioCompat<T> {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl<T: AsyncWrite + Unpin> tokio::io::AsyncWrite for TokioCompat<T> {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    #[inline]
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

impl TokioCompatExt for UnixStream {}
impl TokioCompatExt for TcpStream {}
