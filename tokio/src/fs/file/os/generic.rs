//! OS-independent implementations of asynchronous filesystem functions
use super::super::super::*;

#[cfg(unix)]
use std::{
    future::Future,
    os::unix::fs::FileExt,
    pin::Pin,
    task::{Context, Poll},
};
#[cfg(unix)]
use sys::Blocking;

pub(super) async fn sync_all(file: &File) -> io::Result<()> {
    let mut inner = file.inner.lock().await;
    inner.complete_inflight().await;

    let std = file.std.clone();
    asyncify(move || std.sync_all()).await
}

#[cfg(unix)]
pub(super) fn write_at<'a>(file: &'a File, buf: &'a [u8], ofs: u64) -> WriteAt<'a>
{
    let v = Vec::from(buf);
    let std = file.std.clone();
    let handle: Blocking<io::Result<usize>> = sys::run(move || {
        std.write_at(&v[..], ofs)
    });
    WriteAt {file, buf, ofs, handle}
}

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub(super) struct WriteAt<'a> {
    file: &'a File,
    buf: &'a [u8],
    ofs: u64,
    handle: Blocking<io::Result<usize>>
}

impl<'a> Future for WriteAt<'a> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.handle).poll(cx) {
            Poll::Ready(Ok(r)) => Poll::Ready(r),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Pending => Poll::Pending
        }
    }
}
